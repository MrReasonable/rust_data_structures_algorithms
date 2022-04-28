use std::fmt::{Debug, Formatter};
use std::{collections::BTreeMap, fmt::Display};

use thiserror::Error;

#[derive(Error, Debug)]
enum HuffManError {
    #[error("Attempt to traverse beyond Hufffman Tree leaf")]
    OutOfBounds,
}

enum HuffmanDir {
    Left,
    Right,
}

enum HuffNode {
    Tree(Box<HuffNode>, Box<HuffNode>),
    Leaf(char),
}

struct HScore {
    h: HuffNode,
    s: i32,
}

pub struct HuffEncodedString {
    tree: HuffNode,
    encoded_string: Option<CompressedBoolVec>,
}

#[derive(Debug, Default)]
struct CompressedBoolVec {
    bits_in_last_byte: u8,
    compressed_vec: Vec<u8>,
}

impl HuffNode {
    fn print_lfirst(
        &self,
        depth: i32,
        dir: char,
        f: &mut Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        match self {
            HuffNode::Tree(l, r) => {
                l.print_lfirst(depth + 1, '/', f)?;
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                writeln!(f, "{}{}*", spc, dir)?;
                Ok(r.print_lfirst(depth + 1, '\\', f))?
            }
            HuffNode::Leaf(c) => {
                let mut spc = String::new();
                for _ in 0..depth {
                    spc.push('.');
                }
                Ok(writeln!(f, "{}{}{}", spc, dir, c))?
            }
        }
    }

    fn encode_char(&self, c: char) -> Option<Vec<bool>> {
        match self {
            HuffNode::Tree(l, r) => {
                if let Some(mut v) = l.encode_char(c) {
                    v.insert(0, false);
                    Some(v)
                } else if let Some(mut v) = r.encode_char(c) {
                    v.insert(0, true);
                    Some(v)
                } else {
                    None
                }
            }
            HuffNode::Leaf(nc) => {
                if c == *nc {
                    Some(Vec::new())
                } else {
                    None
                }
            }
        }
    }

    fn encode_str(&self, s: &str) -> Option<Vec<bool>> {
        let mut res = Vec::new();
        for c in s.chars() {
            let v = self.encode_char(c)?;
            res.extend(v.into_iter());
        }
        Some(res)
    }

    pub fn left(&self) -> Result<&Self, HuffManError> {
        self.traverse(HuffmanDir::Left)
    }

    pub fn right(&self) -> Result<&Self, HuffManError> {
        self.traverse(HuffmanDir::Right)
    }

    fn traverse(&self, dir: HuffmanDir) -> Result<&Self, HuffManError> {
        match self {
            HuffNode::Leaf(_) => Err(HuffManError::OutOfBounds),
            HuffNode::Tree(l, r) => match dir {
                HuffmanDir::Left => Ok(l),
                HuffmanDir::Right => Ok(r),
            },
        }
    }
}

impl Debug for HuffNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        self.print_lfirst(0, '<', f)
    }
}

impl HuffEncodedString {
    pub fn encode(s: &str) -> Self {
        let mut map = BTreeMap::new();
        for c in s.chars() {
            let n = *map.get(&c).unwrap_or(&0);
            map.insert(c, n + 1);
        }

        let mut tlist: Vec<HScore> = map
            .into_iter()
            .map(|(k, s)| HScore {
                h: HuffNode::Leaf(k),
                s,
            })
            .collect();

        while tlist.len() > 1 {
            let last = tlist.len() - 1;
            for i in 0..last - 1 {
                if tlist[i].s < tlist[last - 1].s {
                    tlist.swap(i, last - 1);
                }
                if tlist[last - 1].s < tlist[last].s {
                    tlist.swap(last - 1, last);
                }
            }
            let a_node = tlist.pop().unwrap();
            let b_node = tlist.pop().unwrap();
            let nnode = HuffNode::Tree(Box::new(a_node.h), Box::new(b_node.h));
            tlist.push(HScore {
                h: nnode,
                s: a_node.s + b_node.s,
            });
        }
        let tree = tlist.pop().unwrap().h;
        let encoded_string = CompressedBoolVec::compress(tree.encode_str(s));
        Self {
            tree,
            encoded_string,
        }
    }

    fn string_as_digits(&self) -> Option<String> {
        self.encoded_string.as_ref().map(|cbv| {
            cbv.decompress()
                .iter()
                .map(|b| match b {
                    false => '0',
                    true => '1',
                })
                .collect()
        })
    }

    pub fn decode(&self) -> Option<String> {
        match &self.encoded_string {
            None => None,
            Some(v) => {
                v.decompress()
                    .iter()
                    .fold((&self.tree, Some("".to_owned())), |acc, val| {
                        let next_node = match val {
                            false => acc.0.left(),
                            true => acc.0.right(),
                        };
                        match next_node.unwrap() {
                            HuffNode::Leaf(c) => {
                                let s = match acc.1 {
                                    None => None,
                                    Some(mut s) => {
                                        s.push(*c);
                                        Some(s)
                                    }
                                };
                                (&self.tree, s)
                            }
                            node => (node, acc.1),
                        }
                    })
                    .1
            }
        }
    }
}

impl Debug for HuffEncodedString {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "Tree:\n {:?}", self.tree)?;
        writeln!(f, "Encoded string raw: {:?}", self.encoded_string)?;
        writeln!(
            f,
            "Encoded string in binary: {:?}",
            self.string_as_digits().unwrap_or_default()
        )?;
        writeln!(f, "{}", self)?;
        Ok(())
    }
}

impl Display for HuffEncodedString {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(
            f,
            "Decoded string: {}",
            self.decode().unwrap_or_else(|| "".to_owned())
        )
    }
}

impl CompressedBoolVec {
    fn compress(v: Option<Vec<bool>>) -> Option<Self> {
        match v {
            None => None,
            Some(v) if v.is_empty() => None,
            Some(v) => {
                let total_bytes = (v.len() as f32 / 8f32).ceil() as usize;
                let mut compressed_vec = Vec::with_capacity(total_bytes);
                let total_bytes = total_bytes - 1;
                let mut bit = 0;
                let mut i = 0;
                let mut bits_in_last_byte = 0;
                for val in v {
                    if i == total_bytes {
                        bits_in_last_byte = bit + 1;
                    }

                    if bit == 0 {
                        compressed_vec.push(0);
                    }

                    if val {
                        compressed_vec[i] |= (1 << bit) as u8;
                    }
                    bit += 1;

                    if bit >= 8 {
                        bit = 0;
                        i += 1;
                    }
                }
                Some(Self {
                    compressed_vec,
                    bits_in_last_byte,
                })
            }
        }
    }

    fn decompress(&self) -> Vec<bool> {
        let v = &self.compressed_vec;
        let last_byte = v.len() - 1;
        v.iter()
            .enumerate()
            .flat_map(|val| {
                let mut ret = Vec::new();
                let lb = if last_byte == val.0 {
                    self.bits_in_last_byte
                } else {
                    8
                };
                for bit in 0..lb {
                    let val = *val.1 & (1 << bit);
                    ret.push(val != 0);
                }
                ret
            })
            .collect()
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn print_htree() {
        let s = r#"Lorem Ipsum
"Neque porro quisquam est qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit..."
"There is no one who loves pain itself, who seeks after it and wants to have it, simply because it is pain..."

Lorem ipsum dolor sit amet, consectetur adipiscing elit. Nam pulvinar justo sit amet tortor porta sollicitudin. Etiam feugiat rhoncus justo a laoreet. In dignissim nisl ante, et ornare tellus aliquet quis. Vestibulum non tellus convallis, sodales elit vel, congue quam. Nulla non tortor pulvinar, auctor massa a, iaculis velit. Fusce in leo nec ex bibendum hendrerit a eu ligula. Quisque vulputate nisl nec mattis aliquet. Sed ultricies fringilla urna, a suscipit libero maximus eu. Curabitur ultricies leo vel leo vestibulum, at dictum arcu blandit. Aliquam nisi velit, feugiat a vestibulum ac, pretium eu erat. Sed eget pretium nisl. Nulla malesuada urna nec odio scelerisque cursus. Aliquam tristique orci diam, vitae dapibus metus facilisis ut. Vestibulum pharetra erat sit amet facilisis pellentesque. Praesent at consectetur libero, ut suscipit tellus.

Cras tincidunt augue vel turpis scelerisque accumsan. Maecenas laoreet, mauris eu eleifend sagittis, lacus lectus suscipit magna, sit amet luctus ante mauris sed diam. Vestibulum elementum et nisi sit amet ultrices. Pellentesque iaculis interdum justo, sed molestie felis ultrices ac. Donec porta rhoncus ipsum et interdum. Ut egestas augue ut blandit venenatis. Duis ac sem a nibh volutpat finibus. Nunc dignissim, nibh vitae tincidunt convallis, nulla libero iaculis dui, sed dictum augue enim sed purus. Nam porta ligula tristique magna congue, faucibus suscipit risus dictum. Vestibulum rhoncus lectus sed quam consequat, ac condimentum ligula egestas. Aenean vitae mattis nisi.

Suspendisse pulvinar facilisis lorem, sit amet gravida orci bibendum in. In ultrices quam non enim mollis consectetur. Quisque venenatis risus eget consectetur consequat. Aenean hendrerit convallis mauris et interdum. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aenean nec eros in dolor congue lacinia quis et elit. Quisque dui nisi, lacinia vitae cursus et, scelerisque nec eros. Interdum et malesuada fames ac ante ipsum primis in faucibus. Donec gravida venenatis tellus, quis lacinia dolor. Pellentesque ac molestie tortor. In ut orci venenatis, rutrum magna vitae, tempor sem. Sed consectetur justo sit amet sapien pellentesque, sed cursus erat fermentum. Mauris ac tortor est. Nam lacus diam, gravida nec rutrum non, laoreet sed ex.

Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Praesent convallis risus leo, vitae varius sem suscipit in. Proin porttitor orci in viverra accumsan. Curabitur tristique nisl eu bibendum euismod. Nulla at porttitor diam, a egestas felis. Cras iaculis at nisi quis sodales. Pellentesque vitae tellus elit. Aliquam in sem non mauris elementum lobortis. Curabitur auctor vehicula mollis. Vestibulum quis sem pulvinar, facilisis ligula vel, convallis nibh.

Duis mattis risus ac felis tempor, et vestibulum lorem luctus. Suspendisse ut pulvinar nisl. Donec tempus lorem ligula, at maximus turpis fringilla in. Etiam at bibendum elit. Vivamus ullamcorper tempus malesuada. Donec hendrerit vulputate arcu vel aliquet. Morbi imperdiet ut lectus vel elementum. Ut tincidunt sapien tortor, eu sagittis eros dapibus eget. Maecenas elit sem, congue quis ante iaculis, accumsan ultricies lacus.

Nulla porttitor neque dui, vel lobortis ex lacinia in. Phasellus hendrerit vehicula diam nec maximus. Donec vel magna vehicula risus blandit vehicula. Quisque bibendum metus eu ipsum volutpat malesuada et a massa. Donec eget orci non felis imperdiet mattis. Nam tristique quam in ante tempus, vitae fringilla neque tempor. Integer vel volutpat arcu. Donec blandit quam ut neque pellentesque rutrum. Quisque convallis orci non nunc sollicitudin, vitae fringilla sem consequat. Aenean non blandit lorem.

Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Sed ut faucibus lorem. Curabitur blandit mauris placerat justo vulputate, in porta leo pharetra. Vestibulum orci arcu, posuere at lacus eu, tincidunt lobortis lectus. Praesent turpis odio, volutpat vitae risus sed, sagittis fringilla diam. Suspendisse potenti. Suspendisse tempor urna nec lacus vehicula, eget pretium diam tristique. Vivamus non nunc suscipit, euismod dui eget, lacinia turpis. Vestibulum vehicula purus vel diam tempus pharetra. Integer id elementum eros, a pharetra turpis. Aliquam erat volutpat. Nunc placerat erat ut purus pretium scelerisque.

Integer a consequat nulla, ac gravida diam. Donec sit amet turpis quis justo porta lacinia. Sed euismod lacinia nisl, quis sodales magna tempus eu. Mauris euismod, urna vitae pellentesque feugiat, orci sem dapibus orci, nec vehicula odio urna et massa. Vestibulum a dui quis sem venenatis porta. In hac habitasse platea dictumst. Nulla porta ultricies lorem nec ultricies. Quisque in tincidunt nunc. Suspendisse potenti. Donec id urna ullamcorper, congue nibh pulvinar, laoreet quam. Phasellus finibus elit in tortor mollis, in dictum lacus rutrum. Mauris sit amet ipsum nec eros dignissim lacinia.

In dapibus tortor sed leo rutrum faucibus. Cras urna dui, iaculis volutpat congue vitae, semper at augue. Aenean arcu risus, varius quis est nec, congue tincidunt eros. Sed a lorem tincidunt, pharetra ligula vel, cursus libero. Sed eget rutrum dui, sit amet faucibus ligula. Pellentesque at ultricies mi. Sed gravida risus non fringilla accumsan.

Nullam dapibus nibh mauris, in eleifend nunc vestibulum eu. Cras egestas at massa eu tincidunt. Duis iaculis ultricies augue et luctus. Ut ultrices odio diam. Morbi congue nunc dictum aliquam porta. Morbi scelerisque commodo elit quis varius. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Duis at vestibulum massa. Donec iaculis justo eu est mollis, at faucibus lacus faucibus. Donec non efficitur felis. Nam convallis tortor sed ligula dapibus gravida. Proin et risus sit amet mi ultricies pharetra. Vivamus vel sem eleifend, lobortis turpis vel, auctor neque. Maecenas condimentum mauris odio, ac dapibus turpis consectetur ac. Suspendisse ornare imperdiet lacus. Suspendisse placerat mauris non risus iaculis, eget convallis est maximus.

Nulla quis nisi nec orci eleifend tincidunt non porta turpis. In sit amet diam sit amet erat consectetur fermentum. Aenean ut hendrerit risus. Aliquam eget felis arcu. Praesent aliquet auctor congue. Vestibulum faucibus felis vel est mollis finibus. Fusce pellentesque, enim sed sollicitudin condimentum, orci nibh laoreet leo, vel faucibus ligula tortor id est. Praesent pulvinar, tortor id congue aliquet, sapien enim vestibulum velit, in euismod neque nunc vel urna.

Nullam quis lorem id augue ullamcorper luctus. Integer maximus elit id tortor ullamcorper euismod. Aenean bibendum tempor mattis. Maecenas mi quam, feugiat at pretium in, egestas ac odio. Maecenas porttitor quam in massa elementum lacinia tincidunt nec dolor. Integer vitae ligula sed ante tempor tincidunt. Aenean dictum turpis vel lacus elementum, eu pulvinar nisi auctor. Mauris tellus enim, elementum quis ipsum volutpat, pretium tempor magna. Sed elit sapien, scelerisque sed rutrum vel, ultrices a augue. Nulla turpis arcu, lobortis a iaculis sed, mollis nec augue. Vestibulum ac neque non massa scelerisque condimentum. Nulla pharetra ut nisi a lobortis.

Etiam in imperdiet ex, quis pulvinar libero. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Aliquam lacinia finibus magna, eu porta sapien mollis eget. Phasellus pharetra neque est, vitae vestibulum lorem luctus et. Donec et neque a massa interdum consectetur sed ut magna. Etiam ultrices non ex id bibendum. Proin velit quam, molestie quis auctor sit amet, ultrices nec nisl. Integer rutrum arcu id libero mollis lobortis non non arcu. In at accumsan quam, ac lobortis neque. Fusce arcu sapien, luctus et vulputate sit amet, molestie ac turpis.

In quam libero, pulvinar at molestie ac, condimentum vel lectus. Sed commodo tellus at mi accumsan mattis. Ut vehicula massa sed malesuada ultricies. Suspendisse quis lectus nisi. In tristique hendrerit eros at consequat. Suspendisse imperdiet est nec nibh feugiat, ut condimentum enim lacinia. Proin ornare ex sit amet lacus sollicitudin molestie. Vestibulum commodo nisi tellus, pulvinar rhoncus diam pharetra vitae. Proin sollicitudin cursus lacus eu interdum. Vivamus venenatis quam a magna sollicitudin, et varius quam faucibus. Morbi felis nisl, maximus vel est rutrum, bibendum vehicula nulla. Proin tincidunt interdum venenatis.

Etiam rhoncus eu massa ac imperdiet. Integer vel lacus nec dolor iaculis feugiat rhoncus at diam. In hac habitasse platea dictumst. Curabitur finibus vel mi et imperdiet. Cras lacinia fringilla neque quis pellentesque. Ut interdum suscipit dolor, ut mollis nibh. Maecenas malesuada diam in vulputate hendrerit. Nunc ac pharetra enim. Praesent vel libero nunc. Aenean at sodales tellus, pellentesque maximus odio.

Proin feugiat ultrices lectus, at porta mi tincidunt malesuada. Phasellus tincidunt malesuada libero quis sodales. Sed sed nulla nunc. Curabitur id urna vehicula, efficitur nisl vel, pellentesque lacus. Quisque in est ipsum. Duis congue non nunc sed lacinia. Aenean sed massa quis arcu dapibus faucibus in et ipsum. Donec convallis neque nibh, condimentum pulvinar lectus mollis vel. Sed congue gravida tincidunt. Aenean quis sem augue. Nunc faucibus sapien ut augue vehicula, sit amet dictum libero facilisis.

Praesent luctus risus id lorem malesuada, quis pharetra lectus accumsan. Sed vitae ligula iaculis, tincidunt augue sit amet, tincidunt tortor. Nullam tincidunt aliquet nisl eu imperdiet. Nulla interdum diam massa, sed gravida nulla luctus nec. Phasellus condimentum sapien eget congue sodales. Vivamus vel velit urna. Morbi vehicula vel magna non luctus. Nunc volutpat, nulla nec venenatis congue, mi ipsum pretium quam, ac egestas dolor lacus sodales nunc. Pellentesque quis nisl ut sapien semper mattis eu vulputate nisi. Ut mauris quam, laoreet et tortor vel, pellentesque iaculis lectus. Nulla et felis sapien. Cras nunc risus, sagittis vel elit nec, lobortis tempor turpis. Fusce pharetra risus felis, ut sodales massa facilisis vitae. Aliquam tincidunt odio convallis nibh bibendum, quis convallis velit dapibus. Praesent elementum neque in efficitur auctor. Proin nec magna ac mauris viverra fringilla nec sit amet orci.

Ut congue justo elit, sit amet vehicula nulla varius eget. Aliquam vestibulum vel est ut tincidunt. Donec volutpat porttitor erat in scelerisque. Nullam tempus blandit rutrum. Interdum et malesuada fames ac ante ipsum primis in faucibus. In hac habitasse platea dictumst. In dui velit, imperdiet eu gravida vitae, lobortis non diam. Praesent nisi libero, placerat vitae enim ut, malesuada ullamcorper lorem. Nullam molestie bibendum ante, in tempus risus tristique quis. Aliquam posuere ut velit in interdum. Maecenas eu blandit arcu, ac dignissim mi. Nunc et nisl luctus, elementum velit interdum, elementum est. Nam non sapien non diam dictum venenatis.

Morbi commodo tellus lacus, id gravida justo commodo a. Donec gravida hendrerit eleifend. Cras ut tincidunt erat, et blandit metus. Duis quis nibh mattis, rutrum odio sit amet, molestie tellus. Nulla at augue fringilla, maximus lectus vitae, porta velit. Phasellus finibus felis eros, non lacinia felis pulvinar id. Donec ultrices vestibulum ante et consequat. Nullam mi enim, maximus a urna a, pharetra ultrices dui. Quisque sed ligula eget metus dictum tristique. Nam finibus, dolor quis posuere tincidunt, ex nisi gravida ex, sed tristique velit nulla nec neque. Morbi lacinia non sem in pellentesque. Nunc tempor diam at ligula rutrum, ut lacinia dolor suscipit. Proin urna velit, commodo ac commodo id, euismod non dui.

Suspendisse ac elit nunc. Suspendisse ac tristique erat, nec rhoncus nisi. Sed vehicula purus at erat euismod, nec tempor tellus aliquam. Nullam sit amet porttitor sem, ac sagittis mauris. Morbi varius vel dolor vitae euismod. Nullam at quam quis massa efficitur pellentesque vel vel diam. Aenean a nisl in arcu finibus rhoncus. Morbi at elit magna.

Aenean vitae est et sapien tempus suscipit eget quis urna. Proin hendrerit sollicitudin massa at tincidunt. Maecenas ultrices, lacus eu finibus facilisis, nibh urna egestas mauris, vitae porttitor leo felis placerat lorem. Vestibulum pretium aliquet sapien faucibus tempus. Pellentesque maximus eget urna eu ornare. Sed efficitur sem tellus, sit amet feugiat turpis cursus ut. Curabitur dapibus dictum felis, non sodales ipsum condimentum non. Nullam ultricies consectetur nisi, sagittis pharetra felis. Vestibulum molestie posuere ex, vel consectetur felis consectetur non. Praesent et efficitur diam, a tincidunt arcu. Cras a rutrum sem. Morbi felis magna, convallis at tincidunt vel, vulputate id quam. Etiam nec urna interdum, malesuada justo nec, mollis nisl. Vivamus volutpat, ex sit amet tincidunt rhoncus, magna nisl malesuada dolor, ac pharetra nibh velit vel nulla. Fusce condimentum malesuada placerat. Suspendisse ultrices pharetra ligula, ut commodo sem vehicula vel.

Nulla mollis dolor in turpis mollis tempor. Duis vitae quam sit amet magna rutrum auctor id a purus. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nam lacinia leo sed nisl posuere, id eleifend enim gravida. Cras nisl est, vestibulum in ex vel, faucibus fermentum libero. Morbi a justo quis sapien interdum tincidunt. Mauris accumsan, dui id aliquet posuere, neque ante ullamcorper magna, a pharetra ligula nibh a diam.

Integer ligula dui, euismod sed justo sed, tincidunt blandit ante. Praesent pretium felis justo, ut posuere nibh ultricies eu. Nam vitae scelerisque dui. In hac habitasse platea dictumst. Nullam laoreet lectus sed maximus rutrum. Donec malesuada hendrerit nulla ac ultricies. Integer aliquam odio non lacus tincidunt, vel dignissim dolor mattis. Cras neque enim, facilisis vel orci non, pretium tristique neque. Donec mollis gravida metus a consequat. Nam placerat pulvinar nibh, vitae efficitur felis sodales ac. In convallis tincidunt turpis vitae fringilla. Donec a finibus elit. Integer semper, elit et varius eleifend, justo ipsum rutrum odio, id accumsan risus purus id libero. Aliquam dictum turpis quis dapibus lobortis. Morbi semper scelerisque ipsum, eget convallis ex pretium in. Fusce ac mauris pulvinar, vehicula tortor a, pellentesque odio.

Cras sit amet mauris vel libero consectetur facilisis eget sit amet felis. Morbi sit amet erat vitae lacus tristique consequat. Vivamus tincidunt orci eu mauris posuere, ut suscipit mi facilisis. Maecenas id viverra nunc. Quisque erat ante, finibus et turpis a, finibus lacinia ligula. Donec feugiat ullamcorper mauris et sagittis. Mauris rhoncus vel urna ac dictum. Maecenas sagittis gravida orci, et faucibus risus tincidunt vitae. Cras id ornare purus, a feugiat ligula. Nam vitae dolor vitae ante cursus luctus. Nullam vulputate sapien quis faucibus tempus.

Pellentesque ac eleifend mi, tristique rhoncus mauris. Vestibulum scelerisque lacus vel mauris maximus malesuada. Nullam ullamcorper, sem ut vehicula ultrices, mi nisl dapibus ex, non pharetra urna lectus at erat. Nulla mattis malesuada erat in vestibulum. Donec sodales rutrum lorem at faucibus. Mauris egestas dignissim eros, id ultrices nibh luctus eget. Aenean faucibus nunc tortor. Sed euismod consectetur lectus, pretium tempor nunc ultrices sed. Donec molestie rutrum felis viverra elementum.

Nunc venenatis leo in lacus euismod elementum. In eu purus augue. Mauris vehicula faucibus dolor id vehicula. Nulla pellentesque pretium dolor, varius eleifend ex tincidunt sit amet. Duis enim lectus, consectetur vitae iaculis in, aliquam nec turpis. Proin a ullamcorper lectus. Donec sit amet egestas neque. Quisque et semper mi.

Vivamus augue libero, rhoncus id posuere eget, suscipit at ante. Proin dignissim turpis mauris, ac imperdiet dui ultricies at. Ut aliquam, velit sit amet porttitor dapibus, nunc odio tempor tellus, ac sollicitudin neque velit vitae mauris. Integer lacinia aliquet neque. Aenean consectetur velit ex, vitae dignissim ante congue at. Duis luctus placerat erat fermentum eleifend. Suspendisse eu nulla augue. Nam ultricies nunc a ornare suscipit. Pellentesque eget iaculis sem, et venenatis nulla. Nulla facilisi.

Etiam ultricies cursus molestie. Sed efficitur eros diam, eget sodales dolor rutrum at. Praesent consectetur ornare fringilla. Cras eget mi id libero vehicula eleifend. Duis augue est, iaculis et aliquam in, ornare eget eros. Donec aliquam mauris vitae nibh varius tempus. Sed facilisis felis eros, vel efficitur libero ornare porttitor. Mauris sodales elit in eleifend porta. Donec egestas turpis id massa mollis, nec euismod nisl congue. Vivamus efficitur eros tellus, sed venenatis massa ullamcorper vitae. Nunc sit amet nisl ut libero condimentum facilisis a ac ipsum. Vestibulum aliquam maximus cursus.

In sed justo eget purus venenatis maximus at quis tortor. In in neque volutpat, fringilla purus ac, faucibus arcu. Interdum et malesuada fames ac ante ipsum primis in faucibus. Aliquam eu nunc nec tortor pulvinar cursus. Duis at nisl posuere, condimentum quam sed, vestibulum dolor. Fusce mattis ac nisi eget commodo. Sed pellentesque, mauris ut fringilla efficitur, nibh turpis consectetur metus, eu mattis nunc ante varius dolor. Suspendisse sollicitudin tortor ipsum, eget eleifend velit molestie sed. Donec vulputate scelerisque libero id aliquam. Nulla a malesuada turpis.

Aliquam erat volutpat. Nam porta sem sed fermentum rutrum. Nam quis leo consectetur magna sagittis porta ut molestie sem. Aliquam feugiat augue nec lorem dapibus eleifend. Vestibulum bibendum, velit non imperdiet venenatis, felis sem dictum magna, porttitor volutpat orci lectus nec orci. Donec eleifend arcu eget urna tempus feugiat. Morbi maximus diam eget eros accumsan, ut volutpat ex imperdiet. Phasellus accumsan condimentum gravida. Duis nec elit turpis. Suspendisse eu egestas dolor, at molestie libero. Donec porttitor, mauris quis euismod aliquet, mi diam efficitur tellus, eget placerat arcu lacus vel augue. In vulputate orci ultricies arcu commodo placerat. Nulla nunc leo, pellentesque vitae ex at, efficitur luctus ante. Integer et turpis nisl. Quisque viverra molestie leo sed vestibulum. Duis id ullamcorper eros.

Etiam ligula tortor, pretium vitae dui in, accumsan hendrerit purus. Proin ut lobortis enim. Ut eget porttitor arcu. Fusce tempor urna a molestie lacinia. Ut quis sollicitudin risus. Nunc lacus quam, ullamcorper et felis ac, blandit tristique urna. Mauris vel consequat ligula. Sed ullamcorper non lectus quis vestibulum.

Aliquam tortor nibh, luctus non elementum a, imperdiet ut lorem. Aliquam ultricies vel lectus sed consequat. Vestibulum ut mauris elementum, feugiat erat ac, aliquet felis. Aenean lacinia consequat consequat. Nunc suscipit velit nibh, eu scelerisque sapien pretium sollicitudin. Lorem ipsum dolor sit amet, consectetur adipiscing elit. Cras lobortis lectus nec libero feugiat, non ultrices elit ornare. Phasellus at elementum nulla. Nulla molestie cursus tortor, vel pretium nisl facilisis et. Pellentesque vitae luctus metus, quis bibendum dui. Praesent vehicula nibh malesuada diam pharetra, mattis consectetur diam pharetra. Duis consequat commodo felis, nec gravida dui. Proin at tincidunt felis. Donec nunc dui, varius eget ligula in, placerat porttitor ligula. Proin congue, risus in egestas viverra, nisl augue pellentesque ex, sit amet dignissim enim lorem sed eros.

Nulla mi lorem, eleifend sit amet efficitur eget, eleifend vitae sem. Donec massa leo, euismod ac maximus porttitor, volutpat eget nisi. Fusce feugiat volutpat lacinia. Nulla sed felis quis felis tempor placerat in id lectus. Nulla tempus hendrerit consectetur. Proin aliquet sagittis nulla, sed porta justo blandit eget. Aliquam vel scelerisque sapien, at scelerisque ligula. Donec vitae porttitor mi.

Pellentesque sagittis in elit et elementum. Quisque eu lacus et metus facilisis molestie. Morbi bibendum risus lectus, non cursus leo sagittis a. Pellentesque urna nulla, rutrum a condimentum vel, egestas id massa. Interdum et malesuada fames ac ante ipsum primis in faucibus. Phasellus metus nisl, scelerisque at justo nec, commodo ultrices felis. Cras pharetra neque et imperdiet bibendum. Aliquam sagittis nibh et ligula dictum elementum at eget erat. Fusce semper sodales neque id viverra. Aenean sit amet iaculis erat. Nunc elementum urna eget iaculis molestie. Quisque dui lacus, auctor sed molestie vitae, sollicitudin porta augue. Nunc porttitor lacus sed elit condimentum tempus. Suspendisse pretium convallis odio, tincidunt accumsan erat placerat sit amet.

Proin sit amet erat orci. Cras volutpat, dolor dignissim egestas dictum, est elit ultrices orci, eu interdum justo purus in nunc. Praesent augue libero, mollis a nulla nec, cursus ultrices risus. Praesent iaculis nibh nec molestie dictum. Aenean sed nulla ut ex porta dictum non sed diam. Curabitur nec porttitor lorem, quis mollis augue. Pellentesque ultrices semper lorem, aliquam commodo urna ultricies efficitur. Vestibulum posuere nisl tellus, molestie vehicula diam molestie vel. Aenean tellus quam, viverra a lacinia vitae, aliquam vitae ligula. Donec finibus dignissim condimentum. Morbi pellentesque, eros sed malesuada tincidunt, libero enim dictum lorem, sit amet tristique lectus nulla ac urna. Aliquam elit mauris, ultrices quis tortor in, convallis consectetur augue. Nullam quam ipsum, ullamcorper in eleifend et, placerat vel lacus. Phasellus quis tempus nunc, sed mattis est. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae;

Nam non mauris dui. Duis molestie bibendum purus a ornare. Mauris ultrices in neque in fermentum. Ut libero urna, elementum ut fringilla fringilla, vehicula ac risus. Sed vitae elementum purus, et consectetur magna. Morbi facilisis vel tortor pulvinar tincidunt. Ut facilisis vulputate tristique. Praesent in turpis a sapien pulvinar hendrerit ut nec odio. Pellentesque eu iaculis metus. Aliquam tortor lorem, pharetra eget libero ut, vehicula rutrum tortor. Vestibulum non purus nec magna ullamcorper efficitur id ut ante. Aenean tincidunt volutpat augue, vel placerat augue sollicitudin non. Vestibulum elementum in orci quis bibendum. Cras fermentum dui eget neque faucibus congue. Phasellus egestas ligula sed turpis pharetra, a placerat diam vestibulum. Nullam ultrices erat purus, quis imperdiet justo tempus euismod.

Aenean eu felis libero. Aliquam luctus convallis magna, sit amet commodo justo. Aliquam auctor metus in nisi malesuada, id aliquam justo dapibus. Fusce vitae magna non mi lacinia malesuada ornare ut neque. Cras ultricies dignissim fermentum. Nulla hendrerit sapien tellus, non venenatis augue iaculis vitae. Sed at cursus erat, vel rutrum ipsum.

Interdum et malesuada fames ac ante ipsum primis in faucibus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Phasellus sed commodo massa, quis dignissim odio. Curabitur tincidunt turpis dapibus nisl pulvinar dictum. Etiam sed feugiat massa. Cras convallis vestibulum sagittis. Suspendisse quam dolor, dictum id leo sit amet, tempor egestas risus. Aenean hendrerit dignissim dolor, vitae pulvinar ligula interdum sit amet. Mauris euismod erat non diam semper, id mollis lectus pretium. Vestibulum diam dolor, sollicitudin eu orci at, fermentum faucibus lectus. Aliquam tristique quam nisi, at faucibus eros blandit nec.

Donec aliquet dui at pretium volutpat. Proin vestibulum mi a nisl gravida malesuada. Fusce sodales tristique justo eu efficitur. Fusce sed placerat augue, eget commodo nisi. Mauris vel risus sit amet magna pretium aliquet. Integer in urna quis magna tincidunt pulvinar. Duis urna orci, efficitur a accumsan vitae, posuere sit amet nisl. Nullam nisi ligula, tincidunt in tortor sit amet, luctus pulvinar neque. Curabitur efficitur augue sem, quis finibus turpis posuere nec. Ut nec lorem commodo, vehicula sem ut, imperdiet nulla. Etiam accumsan sodales tempus. Nam eget urna vitae metus venenatis hendrerit vitae sed turpis. Vivamus cursus pulvinar est, vel ultrices est aliquet in. In a eros velit. Praesent vitae sodales massa. Nulla mattis dignissim lorem quis sodales.

Fusce vitae velit mollis, volutpat est at, luctus ex. Nam fermentum ullamcorper sodales. Proin nec urna varius, scelerisque nibh at, fringilla justo. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Duis ac nunc quis augue aliquam volutpat at et turpis. Maecenas nisi metus, porttitor id mattis ac, aliquet in metus. Quisque imperdiet iaculis maximus. Suspendisse ultrices suscipit lectus, vitae sollicitudin ex vehicula at. Interdum et malesuada fames ac ante ipsum primis in faucibus. Vestibulum a metus rhoncus, cursus nunc dapibus, facilisis lectus. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; In sollicitudin vitae felis quis porta. Nam eu diam mollis, facilisis leo placerat, rutrum enim. Donec consectetur turpis libero, mattis ornare ipsum aliquam eget. Curabitur dapibus maximus fringilla.

Quisque convallis mattis augue in auctor. Aenean vel ex erat. Ut lacinia lobortis erat sed auctor. Cras malesuada efficitur imperdiet. Vestibulum ante ipsum primis in faucibus orci luctus et ultrices posuere cubilia curae; In commodo vitae est ultrices bibendum. Donec pharetra nec turpis id aliquet. Nulla tristique sodales consequat. Quisque et fringilla tellus, a pulvinar orci. Phasellus ipsum elit, commodo auctor condimentum sed, cursus maximus elit.

Nullam laoreet aliquet sem, eget faucibus diam vulputate et. Duis blandit venenatis odio, et tempus nisi rutrum sit amet. In volutpat ex nisi, ut condimentum quam ornare a. Praesent laoreet justo vehicula, faucibus nibh a, porta ante. In non dolor non urna mattis placerat id a dolor. Nulla eget ex lacinia arcu finibus tristique. Curabitur maximus egestas ex, ut dictum nisi maximus non. Vivamus at porttitor risus. Nulla tempor lorem nisl, ut mollis lacus aliquam et.

Suspendisse massa quam, accumsan sit amet condimentum in, molestie ut nulla. Nulla posuere eros ante, nec dignissim ligula gravida non. Ut eget tellus euismod turpis facilisis tempus. Aliquam sed aliquam sem, quis vulputate tellus. Sed vel massa sed sem gravida imperdiet in ut felis. Nulla feugiat orci eu porta fringilla. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos.

Donec sit amet diam efficitur, ultricies dui ac, luctus neque. Vestibulum leo purus, vestibulum ac vestibulum quis, faucibus id elit. Curabitur venenatis facilisis viverra. Phasellus tempus lorem id justo iaculis, vel mollis augue iaculis. Donec eget rhoncus mauris. Nullam a posuere odio. Sed sodales elit lorem, sit amet condimentum orci scelerisque ultricies. Donec finibus vitae nisi a luctus. Curabitur non metus nec diam blandit pretium. Nullam sed pharetra orci, et lacinia ipsum. In hac habitasse platea dictumst. Praesent auctor viverra libero, nec laoreet libero commodo vel. Maecenas faucibus non nulla quis dictum.

Nulla sit amet metus tempus ex interdum vestibulum. Donec posuere nisi eu ipsum viverra, placerat scelerisque magna cursus. Duis malesuada, nisl in commodo tristique, diam neque pellentesque neque, in viverra nisl velit id tellus. Pellentesque dictum laoreet sapien, ut ornare metus semper nec. Nullam vitae ligula risus. Maecenas rhoncus blandit interdum. Nunc nibh sem, feugiat eu condimentum at, elementum mattis erat. Nunc iaculis iaculis dui, cursus ornare dui fermentum sed. Donec tristique, felis et eleifend maximus, eros enim volutpat arcu, id tristique lectus ipsum vitae tellus. Fusce varius suscipit metus nec imperdiet. Maecenas dolor lectus, tempus ac elementum eu, semper ac risus. Nulla ornare, nisl quis venenatis pulvinar, lorem mi fermentum quam, a volutpat mi nisi sit amet metus. Cras eu quam neque. Duis ut condimentum neque, et mattis felis. Pellentesque finibus, enim in pulvinar placerat, diam odio vulputate ex, sit amet luctus diam tortor sit amet nunc. Nullam mattis libero nunc, ac interdum risus maximus posuere.

Nullam rhoncus metus non lacus tincidunt, at vehicula lacus lacinia. Vivamus interdum nisi at congue faucibus. Duis venenatis dictum tempor. Integer id est a ex interdum egestas. Curabitur eu augue nunc. Suspendisse id ligula neque. Ut feugiat et mi eget porta. Duis tincidunt maximus turpis, sit amet semper orci rutrum quis. Maecenas sagittis enim dolor, a mollis tortor aliquet imperdiet. Pellentesque rutrum sollicitudin commodo. Pellentesque elementum libero id cursus facilisis. Suspendisse a vulputate nunc. Aenean placerat, sapien eget efficitur pretium, risus nulla tempus risus, non malesuada ante enim quis dolor. Donec a eros nunc. Etiam ultrices convallis risus, id ultricies dolor viverra quis. Phasellus non cursus diam, non porta magna.

Proin sed tincidunt arcu. Nulla tortor nunc, consequat quis placerat a, bibendum nec tellus. Cras ac rutrum est. Suspendisse egestas pellentesque orci, in commodo nisi sodales sit amet. In tortor erat, pretium vel leo quis, vulputate vulputate purus. Sed ac pharetra ante. Cras et dictum metus. Cras posuere fermentum est sit amet feugiat.

Quisque molestie dapibus felis, sed pretium nisl vulputate non. Sed tincidunt ac arcu vel suscipit. Mauris auctor, dui ut viverra accumsan, erat tortor egestas augue, id bibendum erat magna in leo. Praesent nec libero at risus vulputate aliquet. Fusce et neque fringilla, scelerisque ex eget, pulvinar lectus. Etiam condimentum ante ut neque egestas tempus. Nunc consectetur egestas eros in accumsan. Curabitur id erat aliquam, gravida dolor non, maximus lectus. Interdum et malesuada fames ac ante ipsum primis in faucibus.

Etiam scelerisque orci sit amet massa imperdiet fringilla. Vestibulum velit ipsum, luctus a eleifend at, suscipit vel elit. Aenean non rhoncus urna. Maecenas semper egestas est, a consequat metus cursus non. Ut tincidunt est ut mi fringilla, ac finibus leo sollicitudin. Nunc tempor nulla sit amet neque porttitor venenatis. Nunc congue leo vel arcu ornare rhoncus. Maecenas vehicula, est nec mollis tristique, sem quam vulputate leo, vitae pretium ipsum metus id dui. Sed vel sem mollis, semper lacus ac, vehicula metus. Aliquam accumsan, ante eu luctus elementum, sem arcu eleifend ante, ac interdum urna nulla eu mauris. Aliquam tempor pulvinar finibus. Sed tincidunt maximus tortor id vulputate. Phasellus consequat libero at metus efficitur sollicitudin. Praesent porta mattis ultrices.

Morbi eget ullamcorper felis. Cras et purus at sem venenatis dictum. Nunc varius egestas aliquam. Sed lacinia dui nulla, ac tincidunt nibh aliquam ornare. Curabitur magna magna, molestie rutrum est id, convallis commodo nisl. Ut elementum auctor mi, sed porttitor elit luctus a. Aenean at lectus nisl. Sed aliquet pellentesque suscipit. Aliquam urna nibh, pellentesque vel elementum varius, dictum sed massa. Integer enim felis, faucibus ut risus quis, gravida venenatis massa. Sed quis mi hendrerit, facilisis felis in, venenatis justo. Sed tincidunt diam a fringilla euismod. Nullam sit amet nisi aliquam, facilisis lacus in, blandit ante. Maecenas vitae tempus lacus. "#;
        println!("{}", s);
        let t = HuffEncodedString::encode(s);
        println!("{:?}", t);
    }
}

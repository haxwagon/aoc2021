use std::collections::HashMap;

use crate::parsing;

pub fn run() {
    let digit_counts = count_digit_segments(&get_digit_segments());
    println!(
        "Day  8: There will be {} 1s, {} 4s, {} 7s, {} 8s for a total of {}",
        digit_counts[1],
        digit_counts[4],
        digit_counts[7],
        digit_counts[8],
        digit_counts[1] + digit_counts[4] + digit_counts[7] + digit_counts[8]
    );
    println!(
        "      : Total of outputs={}",
        decode_segments(&get_digit_segments()).iter().sum::<u32>()
    );
}

pub fn count_digit_segments(d: &Vec<(Vec<String>, Vec<String>)>) -> [u32; 10] {
    let mut digits = [0; 10];
    d.iter()
        .flat_map(|(_input, output)| output.iter())
        .map(|entry| match entry.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => 0,
        })
        .for_each(|num| digits[num] += 1);
    digits
}

pub fn decode_segments(d: &Vec<(Vec<String>, Vec<String>)>) -> Vec<u32> {
    //  0
    // 1 2
    //  3
    // 4 5
    //  6
    d.iter()
        .map(|(input, output)| {
            type Options = [u32; 7];
            let mut opts: Options = [127; 7]; // what segment options remain possible by letter
            let build_number_by_segments = |x: &[u32; 7]| -> HashMap<u32, u32> {
                // builds a mapping from segment identifiers -> number
                let mut mapping = HashMap::<u32, u32>::new();
                mapping.insert(x[0] | x[1] | x[2] | x[4] | x[5] | x[6], 0);
                mapping.insert(x[2] | x[5], 1);
                mapping.insert(x[0] | x[2] | x[3] | x[4] | x[6], 2);
                mapping.insert(x[0] | x[2] | x[3] | x[5] | x[6], 3);
                mapping.insert(x[1] | x[2] | x[3] | x[5], 4);
                mapping.insert(x[0] | x[1] | x[3] | x[5] | x[6], 5);
                mapping.insert(x[0] | x[1] | x[3] | x[4] | x[5] | x[6], 6);
                mapping.insert(x[0] | x[2] | x[5], 7);
                mapping.insert(x[0] | x[1] | x[2] | x[3] | x[4] | x[5] | x[6], 8);
                mapping.insert(x[0] | x[1] | x[2] | x[3] | x[5] | x[6], 9);
                mapping
            };
            let is_found = |x: u32| -> bool {
                x == 1 || x == 2 || x == 4 || x == 8 || x == 16 || x == 32 || x == 64
            };
            let map_segments = |x: &String| -> u32 {
                // maps the input string to a number where each bit represents a letter
                x.chars()
                    .map(|c| match c {
                        'a' => 1,
                        'b' => 2,
                        'c' => 4,
                        'd' => 8,
                        'e' => 16,
                        'f' => 32,
                        'g' => 64,
                        _ => 0,
                    })
                    .sum()
            };
            let restrict_opts_based_on_found = |options: &mut Options| {
                // if any numbers have been found, restrict remaining options
                let mut changed = true;
                while changed {
                    changed = false;

                    (0..7).for_each(|i| {
                        if is_found(options[i]) {
                            (0..7).for_each(|j| {
                                if i != j && options[i] & options[j] > 0 {
                                    options[j] &= !options[i];
                                    changed = true;
                                }
                            });
                        }
                    });
                }
            };

            // limit what segment options are valid by the obvious numbers with unique segment counts
            input.iter().chain(output.iter()).for_each(|s| {
                let segments = map_segments(s);
                match s.len() {
                    2 => (0..7).for_each(|i| {
                        if i == 2 || i == 5 {
                            opts[i] &= segments;
                        } else {
                            opts[i] &= !segments;
                        }
                    }),
                    3 => (0..7).for_each(|i| {
                        if i == 0 || i == 2 || i == 5 {
                            opts[i] &= segments;
                        } else {
                            opts[i] &= !segments;
                        }
                    }),
                    4 => (0..7).for_each(|i| {
                        if i == 1 || i == 2 || i == 3 || i == 5 {
                            opts[i] &= segments;
                        } else {
                            opts[i] &= !segments;
                        }
                    }),
                    7 => {}
                    _ => {}
                }
            });
            restrict_opts_based_on_found(&mut opts);

            vec![
                (3, 1), // disambiguate 0
                (2, 5), // disambiguate 6
                (4, 6), // disambiguate 9
            ]
            .iter()
            .for_each(|(good, bad)| {
                if opts[*good] != opts[*bad] {
                    return;
                }
                let mut i = 0;
                while i < 8 {
                    let check_digit = u32::pow(2, i);
                    if opts[*good] & check_digit > 0 {
                        let check = 127 & !check_digit;
                        if input
                            .iter()
                            .chain(output.iter())
                            .map(map_segments)
                            .filter(|x| *x == check)
                            .count()
                            > 0
                        {
                            // we know that check is a valid digit, thus check_digit is not a
                            // possibility for opts[good]
                            (0..7).for_each(|i| {
                                if &i == good {
                                    opts[i] &= check_digit;
                                } else {
                                    opts[i] &= !check_digit;
                                }
                            });
                        }
                    }
                    i += 1;
                }
            });

            // map output digits to a base 10 number resolved by mapping segments
            let number_by_segments = build_number_by_segments(&opts);
            let mut output_value: u32 = 0;
            output
                .iter()
                .map(map_segments)
                .map(|x| number_by_segments[&x])
                .for_each(|x| {
                    output_value *= 10;
                    output_value += x;
                });
            output_value
        })
        .collect::<Vec<u32>>()
}

fn parse_digit_segments(s: &str) -> Vec<(Vec<String>, Vec<String>)> {
    let mut entries = Vec::new();
    parsing::parse_input(s, |parts| {
        let mut parts = parts.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        if let Some(slash_at) = parts.iter().position(|p| p == "|") {
            let mut output = parts.split_off(slash_at);
            output.remove(0);
            entries.push((parts, output));
        }
    });
    entries
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_digit_segments() {
        let digit_segments = parse_digit_segments(
            r"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ",
        );
        let digits = count_digit_segments(&digit_segments);
        assert_eq!(digits[1], 8);
        assert_eq!(digits[4], 6);
        assert_eq!(digits[7], 5);
        assert_eq!(digits[8], 7);
    }

    #[test]
    fn test_decode_segments() {
        let digit_segments = parse_digit_segments(
            r"
            be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
            edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
            fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
            fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
            aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
            fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
            dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
            bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
            egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
            gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
        ",
        );
        assert_eq!(decode_segments(&digit_segments).iter().sum::<u32>(), 61229);
    }
}

fn get_digit_segments() -> Vec<(Vec<String>, Vec<String>)> {
    parse_digit_segments(
        r"
cgaed gcdbfa gcfaed gfcde gadfceb cdbfeg acg eacf eabgd ca | agc efcgbd cag eacf
ga ega edgfa cafed gabd cefagdb begfad ebdgf fcbega cbgdfe | bgdef fdgeb dgabfe gea
ged eg acfgd fdceb cdbefa dgcfe cebfdg edcbga egbf ceadfbg | dfcge dacegbf gcdbaef fdceg
cbefg fedbcg bfdg abgedc fgced edcagf caefb gb fedcbga beg | bdfg cbeaf cfdebg gbe
gecdab gbafd geabdcf ecfa dbcfge cfb afgbc efgbac cf cgbae | fbc fbc cfebgad cf
cbdef febdga fba ba fadecb cgfea adbc cbafe fbedcag egcfbd | dfgbae fdcageb adcefbg efbdc
gecdba ecafbg ecabg bgacfd gecaf feba aedbfgc fca gdfec af | fac af cfa eafbgc
geab adgfc gacef acgefdb gecfba eg dbfgec beacf afedbc ecg | baeg fgadc fagbce edfgbca
gebcfad dafceg gdcaf bdfgea ab befgc gfbca bdca gab dgabcf | bga fgceb bag gfeabd
efgcdb egd gceaf dabgcf ed acegd gdbaefc dabe dgacb adcebg | afecg ed de egcfabd
eb efb eafgbd bgce adcfe fbcgd fgdbac bdefcg fcdbe fcebadg | ecgb bcdagef bcge egfcbad
cafgbde aced gfdaec defgbc ce gec gfdca bfgacd afgeb cegaf | cafegdb efabg cfbged eafgdcb
gcedba dgbe eba ecadgf bcfda bfegca eb gfecadb abcde egcad | eb be bea aeb
fgcd bcf edfbg dbgeafc fc efdbc fcdbeg beacd adgebf aebfcg | cf dgcf cf dceab
decag bade ad gda gadceb fgcbae adfgcb dfecg gceab edfgcab | adg gbcae gda da
adbgfe fd dbgf gdeaf agfeb agcde gedabcf afcedb gbceaf daf | afdeg bagefc afd bagfecd
cgeabfd efbgac dgcfba gbfd dcageb dga gd bgcaf fgadc aecfd | fbdg agd fgcbade gdfb
efdgc gfdb cegbfd bdefcga bcgef bacdge eafcd afgecb cdg dg | gd dgfce dg dcg
edgabc cgaf gdbfa cafdgeb dbfacg gdc adfebg fgbcd gc bcdef | bgacfd cdg ebfdc cafg
bcdfage ebgdac dcb adce cd acdgb bcfedg gbace dbfag fgceab | dcebfga caed eacd bgceda
fd gafcdb feadcgb dacefb gdfab fagcbe gcdf bdf edagb gacfb | dfb gcfabe bgdaf bfd
daefcb cg gfdea ceg ecfabg beafc gecdafb eafcg bcfg abdcge | gec egafc eabcf edbgac
degafbc fgeac gb acbeg bcead dcafge bcfg gbe bcgefa edafgb | fbaecgd bcgf beg bg
adeg ga agfcbde gadefc fbcged acg gefdc gcbeaf acdbf acgfd | cebgfda cag cfegab gac
bfegda eag dbcafg gfedabc fegcab afgbd ae faed dbaeg bdecg | fadgcb afed bgadcfe ecgabfd
bcdga bgd cdfag dbaec dcfgeab cadfeg gb cdgbfa fdcebg gbfa | bdg bg adcefbg efagbcd
acfgbed eagdcf cgdb db gdcbfa acbdf cbefa gadfc gbadfe bfd | bfd acfbd db gfdeac
agbed geaf cfbgaed dae ae deafcb fbcdeg dgefb dgcab fbegad | defabg cbdafeg febdcga acbgd
bcade fdecba bgceaf acg ecdfg adbg cgfdbae bcgead ga eacgd | cga afdceb cdgbea ga
agcefb cbfdaeg fabce eg dcgfb bcadfe gace bedagf gbefc beg | fegabd ceag ge dbeafc
efbgc ebgfa dacegfb daegf bcfgea dbcgef abfc agbecd ba bag | ab ab dcegbf cbaf
aebf fed fdbacg fe gdeacf egdfb ecgfabd gcdbe fbaedg fabdg | def aebf fe ef
adf cgabfd daegf af edfgc egbcfad gcbdae ebaf gbeda faedgb | feba eafb feba abgcdf
gab gcfb cgbdae defab afbdg acefdg bfgdac bg eafcbdg cagfd | bfgc fdbgeac cbaegd cfbg
eagfbd fcbgd baceg dba bcdgfe facgbd da dafc cgafedb bcdga | fdca afdc cbfegd gebfdac
feacgb gbfac dag dbac cdfgea gfebacd gfdeb gdfba da dfgcab | dga aedgfbc ad cdba
cfdabg ecag ecgadb eg ebg faebd dgbea dacfegb gbdca bgfecd | dgcefb baedg ecbafdg bge
bfdec fgdbce debga adecfb fa face dgfabec bfdae gacbfd bfa | afec af fa cfea
fcbad ecd fbagecd deabg gaedbc fgbdae ec adbce aecg ebfcdg | ec gcea ced ec
dg efbcgd cfdbaeg cfdga adeg bagcfe caefgd gdf feacg cdfab | gaedcbf abcfd dg dfegca
fdgbe gbc egfcdb cbdeg dcbf ecgad bc gcbafe beadfg bgdface | bfcd bfdc gcebfda bc
gace gcefb cgdebf ac cfa fbeac fgcdeba ebafd bagcdf gfacbe | ac feacdgb bcfea gafbdce
fcbega dfgcab gacbedf fgeca cbeg gc fegdba gac ecfda egabf | cbge ebgc gdebacf gc
cbga bdceaf bgfdec ba efgab aeb efcbga gdaef egbcf abgcdef | bacfde cgafbe dbcgaef fcgebda
fdcba bgfcd fdbaecg fcgaeb cegd bfgdae dgefb dfcgeb gc gbc | degc cgb cegd gdbaefc
caedgf bge dacbg be dbfecag cgbefa eagfc ebaf ecbag fdcbeg | beg eb agceb egfdac
dacgbef gebdfa bdcfga cgd adfgec cd cdba cdfbg ebcgf fdgab | defcga bcda acbd bgefc
fdgac cebf baedc dceaf adfcbe bfedga edgcba fe fed edabfgc | adcfg dagefbc egbcda fdceba
dagfbc fceabd fgbc edbcafg bdf bf bdgae afgcd fbdga gaefdc | gdcbfa bf bf fbcg
dgfcb abdfe fgadbc cfbgae gadfb afg cdgbefa cagd fcgdbe ag | gaf dabegfc fadgb ebacdfg
gbedc faced dfb fbegadc fcdbe fcdgab bf bdafce beaf edfacg | bf fb bf dfb
aegcfb dacgbfe fc ebcag acgebd acef cfbag cgf cbedfg fagdb | acgeb cbega bfdcgea debfgac
dgcfabe bgad eab agedc befcg cbaegd cefabd gaecb dceagf ab | bagd agdfce bea ba
fgbde fgba efb bf fcdgae afdeg afbecd becgd efdcbga dfegab | bgaf fb bf gefad
fb acgefb egafd baf adefb dcageb bdafce aefgdcb decab cbdf | dfcb afb fb begfadc
edcaf dgabfe ceb bgdc cb gaefcb fdgceb ecdfgba begdf bdcfe | cb gfdceab dbcfe bc
gedfcba bcdaeg dgae gca gdcbe afbdgc gfdcbe agcbe eafcb ag | gcbeafd agc bdegafc daebcg
aebcfg fagdc fcgea fgdebca ecabf ge feg bcge fgadeb cdebaf | fgeca ecgb eg gcafedb
aegcf becgdaf cbfag bafcd gcb cfgbde abdfce acdgfb bg dgba | egfcabd gb abdg gfcae
egcab adfegb gc febgac dbcea fgce bgafcde cdgbaf fbgea cgb | fgce bgc gc cg
cabfe dfa agdec df aecdfb cfbadg fagceb befd cfade dbaefgc | bcfgad faebcg df df
gcab fbeadcg badcf ag fgaced gfabdc dgbfa fag bgfed afdbec | fabcd cdbfgea cgefdba gaf
ae bdefac dacbf ecabd afdecgb feacbg eba aedf bcgde dfgcab | bae edaf ae ea
fdgba bdgeca cgb fdgec gbcadef dgabef bcagdf bc fbac gbcfd | cbg bc cfbgd dfcagb
afdgbce geac egfdca acbedf fgdce faced eg efg dgbaef bdcgf | dcfbeag afbegcd ge cega
daf fdaeb df dbfg dbeca egfab cegabf cfeadg cbdaefg dfbaeg | fda agfbe dfgb afd
gdcabe bfc dgbefca fc aebgc dcabfe efbgac fbcga agbfd efgc | bacdeg bfc cf bcgdeaf
cf fgdbe deacfg dbcea efdcb bfcg fdebcg fdc befdag gebdfca | cf cgbf fdegabc defbgac
gdfba fceadg cb fbcgd cgeb cdefg cdb bdfcae fgdcbe gbedfca | dbc cb fgbdc dfcagbe
dbgcaef dbfc eafgb fd dgbaec bcdag daf dfcaeg bdgfa bfcagd | adf fd df cdaefg
cd cfedb cgadebf efbgc abdfec edfab cadefg abcd dcf gbeadf | dc debcf fdc fbagcde
cgdfbae fe gecfd defb adcgeb caefgb fge dcgeb cgedfb cdfga | fe fge befgac fedb
egadb bdfge gcebdf ga cfdgabe edabc gdfa beafgd abg bacgef | fdga fbcdeg fedagbc bag
gbdea bfaeg fba gabcdf gadbec bf dbef egabdfc cfeag dfbgea | fb bacegfd dfeb bf
ebdac becfda bd bedfga edgcafb bed aecdg ebafcg dbcf abfec | cdfeab dbe cdfb ecabfd
gfbac dcage dfgace abgfed fgcdaeb fd agcebd defc gafdc gdf | ebdagfc gdf cdef dfg
bcfdeg dgceaf cea fecgb cbafe ea geab adcfb bdfgcae gcefab | egbcfa geba gdecaf caebf
gd agebc fgcd dbg fdgecb egfbda fbdgaec fcdeab bfdce bcdeg | caefdbg cagfebd debcfga cdgf
edc acged fcea ce dgecaf ebgdfc bcafgd fdcga acbgedf aegbd | gbfadce gbdefc ce afec
gfebd agbcfd acdbf caefdb abgefc gbfadce abe eacd ae dafeb | ae dbcfgea ea edac
aebcf dea gdacfe gfad adcgbfe caedf gbdeca ad dgcbef cfdeg | ade fdecga ad eda
eabgcd gdaeb faebgcd gaedf fdcbea efd ef efgb fgcad daefgb | gfbedac fecabgd fceabd dbfaec
cbeaf bafceg bfeg bcf bdcfag cdeaf fb edbcag bgeac cbgefad | abceg fb ebdafgc bf
ced aecgfdb fdecgb gcefa agde ed efdca abfcd begafc agdfce | gead bdacf abfdegc decfa
cda gdcaeb cadfe ebcagf cbefa dfabce gfdae cdfb cadgebf cd | dca cd bfdace aefdg
gacfbde bg degcaf gfbce aefgcb cbdef egcfa acgb bge gfaedb | cagebdf dfaegc ebgfac bagc
aefcd dfgea dfc cf fedcga cbdea adgefb adbgcf gecf gcbefad | cf dfagec fdc edfcbag
gefbd adgf eafdb gd gecfb fedgba bgd acbfde cdfageb adgbec | fbdgaec dg gdb aecbdfg
cdfabe cfga cg dcefa gce eagbd agced fcdeag gcefbd bdaefcg | gaedfc ecadgf gcfa dbega
dgcea cfgebda dagb fgcaed bcg bg bcaef cedabg bcaeg cegfdb | cgbade bfdecag aedgc gfcbdae
dbcfg cdfeg dbf befagc debfac agbd dgbcfa dbfgace db facgb | bd fbd bd bd
dbefga efcadb cgdab cdbfa gad fdcbgea gbaec dg fcdg acfdgb | dg dgcba fgcd dg
ebag gbafed fbdgac eb bdfag bef dfbge abegdcf fedcg acbdef | fgbade gabe edgbf fgcbdea
deg bcgfad efdagb fedagcb eg ebcg cdafe decfg dgcfb dfcbge | cdbgf cgfedba bcge eagbdf
egbdc efbgdc aec bagfecd eagcbd gbface ae adbe fadcg gcead | eac eca egbadc eac
acedb fdcgea degabf ebcafd eb fceb agdbfec deb cgbad feacd | eb edb agbcdef cdabe
gcdbfae eadb feacg ad dbcef dac bcedgf dbacgf cfdea bdafce | fdcbe dca fdebc caefg
edfba cedf cebad ecdfab decabgf acgbe cgabfd ebgdfa adc dc | cad adc baecdf efabd
caeg dbefg egdba afgbcd fbdcea cdgafeb ag agb bdecga adecb | ga gdaefbc ag dbeacg
bf bagdcf gbfa dgaecf gdfcb fdgbcae fdb bfcdea gcebd cdfga | febcda bagf fb fgba
cgbdef begfa agebd bdac da bgeacd ecdgb aed defacg cdbgfea | cbgdef da dbca ad
aebfdg aegbdcf cdebg ebgcdf ef cbef dgbcae gfe gfcda gfedc | gcdbe fge efcdgba fe
befad bcdf fce bgcaefd gaefdb dacfe cbefag gaecd caefbd cf | fce fc fdbc fc
gecab gfeca eafdbcg bgcdae fdcgeb gbc dceafb gb dabg abedc | agbd gafce gcaeb cdbgea
deabf dfg fcedgba bfgc fg cbdeag fedcga fbcadg cbgda afdbg | gbcf bgcad gdbca cgfb
bcafgd gefba acg abec decgf gbacef ca gafdbe cafeg bgecafd | afgec fgecdba gbadfc bgcadfe
febcgda ebga dgbecf efadbg ceafd eb edb bdgfac fbade fgabd | fdgba edb egab fdbga
dgb dbce faebcg bd gfbdac fecgb gbfcade bdgef bgfdce edgaf | bd db dbg bcdgefa
fdaeg afebgc dcf gcafbed cbfge dc dbcg bcfegd cbdefa edgcf | bcafge bagcfe agedf bgdc
cgfb bacgdfe bdfeg fdebcg cb cdb edgfab edfca becdf gebacd | cdefb acdegb bc afdbeg
gbdcaf gfbad gfdabce abfdce debagf gcfad ac gfced fca agbc | fca agcb egfdab gdcabf
cedbagf edagc bge bcdge gbad gb bgefac aecfgd bdfec cegdba | gbefadc febgadc ebg cagde
dfecga cdagbfe aegcd cdfg fegab bcfaed egafc cbegda fc fac | cdfg gdfbcae caf fc
gecdf agdfe dga cfda cegbda bafeg ad caefgd dfagceb bfgdec | dag gefdca cdfbgea afbgdec
deabf gdea bcaedf agdfeb dfg gd dcfabeg ecfbg edbfg bgdfac | gdf ecfgabd badgfc fgd
gfabc gafce adeg bgdfce dabgefc eacdfg ea dfcge ebcafd afe | agcedf fcgedb fcdaeb becgfd
dcbgeaf dbafec cafde beagdc gd dafg cfbge dge fegcda fgedc | dagf dgefc gd agdf
egcbdf dafe dfbge adbgefc eadbg egabc gcfdba abgedf dba ad | begdf efabcgd eafd edaf
cegd bcfde gfbcdea efdab dbcfg cef efbcdg ce abcfgd aecfbg | bedfc bfeda egcd dbefa
abdceg acedg efbcg ab bcad gcabe bdfage afdgce agebdcf gba | gadbcfe gbeac bdgeacf fbgec
dfba fcebad afgebc dbcea fcgdaeb befcd ebf gcdef fb eabgdc | bf efb bef agbfec
cfgde geadb fgdbec gfcb fbd bcfade egdbf adcefg bedcagf bf | bf fcbg bdf gdcefba
edf fgdbce agcfbe efbdga dafge beda de cbgfeda cdagf gafbe | ed gfead fed gcafd
ebdg fcbedg cdbagf de fbedca fecag daegfcb cbgfd cedfg dec | fbedcg cde ecd bgefacd
fbecgd gefca bcefa fgbeda deabgcf gc adgef cgad gfc gcefad | gc gc cfdega fcg
cgefab afb abgfde gadfcbe fbgced dcbga fa gebdf gfbda efad | cegbfa fbegda edgbf baf
dcbfg adce cegbd ed egcdbfa befagd bgace bcfaeg bdacge egd | fgdbeac geabdc begcadf acgefbd
ebgfac dabcfe fea bdafcg ea defbg aecg fcagb cafebgd abfge | gcfab gbfea afe efa
fdb beadfc decgb fd acdf dafbegc cfbed fcbaeg cabfe agfbde | fdb dcegb eabcf efcgdba
cbad dae dcbge feabg da gcbdef gaedb dbcegaf edagbc dgecfa | ade ade da adcb
fbe dcbefg fe adfbce gebdacf fbadcg baceg bdacf fead fceab | debfgc ef ef bcaeg
ab cbgdfae cbdagf dceabg ebad bag gdbce edfbcg cefga ebcag | dabe ba fagdebc gcbde
cdagbf fcadbe gac acgbd bgcde cgbafe eabgcfd ga adgf cbafd | ag agfd adfg cbgafd
edca eagcfbd dbgae dge ed bgcad bdcfga fagbe gbcaed gebdfc | cead gde deg fcdgbe
edb abfgecd edgf fgaeb bdage cadgb fdcbea de fbedga abfceg | eadfcb bedgcfa bed dbe
faedgbc cfebad dbecf afeb ab afdcg dba adbfc aedgcb ebfcgd | dab fcgad dbgefca dcfga
aedc dagbcf afbdce gfbed cd efbac acefgb dcefgba cfebd dcf | fbcagde eadbfcg cd bedfg
agedbc egdcb eb bgea bedfac dbgca cgfde edgabfc cbe dbfcga | ecdgb fdcge bega dfebca
dfbeac gcdefa cf fbcea ebadf efc gdfaeb abgce ebfcdag fdcb | ecf bgeac fc fcbd
gacdef dbeaf dgaefb fc dcafgeb ebcdfa gbecd bcdfe cdf abfc | cdf fdc egfbdca cf
ecfdba be bgce fcedag abdfg gbfed acgdfbe cefdbg cefgd dbe | deb be dfbag cgeb
bfgac acbfgd fga dgbca gbaefd efgcb cdefbga dgeacb af fdca | fadcebg fa adfgbec fa
dbgeac gd decg dabegfc dbeacf eadcb fdbega adgbc gbfac dbg | cdeg dabgc ecdba agcdbe
daebg ecgfd cbfeadg eabf degcba fgbdca afg gfaedb gfaed af | dcabge baef agf feab
edcgab dfcbage bfga becgfa gca efdca gfbdec ga becgf fecga | ebagcf gfba bgaf ebfcgd
bdfeag dcbfeag eg cdge afbec gea ecgaf fgadbc fcgeda fgdac | eg eagfc cdeg egcd
gdbeacf dfcgba ebafd ag bgaecf bcfgd afg gdafb bedgfc dacg | dgac gdca gfa gefbca
afbedc bfd gdbce bedafg gdfeb bf eacbfgd gefad agdefc bgaf | feagd bafdge fbag bdcafe
cae becgad gdabe ac cagb cgafde ebdac fcbde abcdfge degbaf | dbefga adcbge dgfeba fegbdac
caedb adb ecfgdab gdaec gbdeca ab cgdefa fgdcab dcbef eagb | gfaedc ba ba bcgafd
dcefa gcd agdfc bgeadc dg fgceabd dfgaec dfeg gbcaf dbfeac | bcgead fgacebd gcbaed bagcf
adbgce cd cdeg adc bcedgaf fbeac bdace ebdga afbged cbdagf | dac gedc cda dc
ad dfbce cdabf fgbca edbcga fead ebdgcaf bgfedc dac bcfade | eafd ad fgcab fbaecd
bfgdae baegf cbfeagd ebdaf fedca dabg dbe efbcgd db abgfec | bdfeag egbaf db gbda
gcfea eafdb eabgcfd egafb gadfeb gb fdgb bdafec ebg egabdc | decabf acdgfbe dbfg bfgd
gebaf cedgbf acbeg cbagdfe fbe acbf gaefd gcbdae fb gecfba | agfdceb bf bfca acbf
cegabfd fc gaefbd cbdea fadec fecg afc gecadf cbgdaf dgefa | cfbgda fac dfeca abgdcef
ab gdfea egba afegdc fdbae cfedb bad aefdbcg dgfcab egdbaf | facgdb abfedgc ab ba
af fgbeadc ceadg cfeagb fgdbae efa egfad efgdb fdab begdfc | dbfeg fa af bafd
dfgecb cgabf cdaefbg adbcge dabeg bec ec aecbg febdag aced | aebgfcd fgedba aedcgb adecbgf
bcadef cbfgd dcagbf dfe febg befcgd degfc degac bfagced fe | fde efdbgca gcbfdae fgbe
fgd edfc fgedca gafdebc febga agedf eadcg gdebca dacbfg df | cdef dgeca bdcfga efdc
aedcfbg dfceba eb caegdb begca fbacg bea egdb ecagd dgcafe | eba gdeac abe bgde
cg fcdg deacbgf cdgbaf fbcea abfgd gac badceg facgb dbfage | dfcg fabedg gc cgebda
bdgae eg defba gcedab bgadc dbfcag egbc ged gafcde cfbagde | geafbdc bfead ged bgead
agefc cgdef cgfdea ca fcdabg fecdgba aedc bgfae fac dgcbfe | fgecd agefc gafdec ceafg
ca cagbfd efbdc fac afecd aebc cadbef gfaedbc dbgefc feagd | ac dfgcabe defbc cdafbg
cdb ecfab gbcaed cafdg gbfd db adbcf cdagef afgcdb cfdebga | db bcdgae dbc db
begfcd ag cgaebf gfbcdea bdgeaf gead bgadf agf fdgeb acdbf | gfbad ag fadcb ga
ad gbced dab bcfage adbge afegb fcagdeb fbdaeg dagf acbdfe | fadcgeb bedga gdfa gdbeaf
begfdc fgdcb fgadbe caebg cagfdb efg edcf ebcgf ef efbdgca | degafb gef fagbcd feg
ecfdg gfbed gdac acgbfe cdafe bafdgce gec dfecga cg afcdbe | cg gc abcdfge ceg
deabg gea egdfcab bgcda gbfe fdbgae efcdag fadbe eg efadcb | gafbcde ega gfeb dfgbcae
ef gecadb adbeg defbg fcbgd fgae feb efgabd bedfca facbdeg | efb bef ef agef
cefga cfd fcebdg ceagbd bfdg bfdeac gcbde gfdce agcdfbe fd | faedgcb fdc dcgfbe cdf
dfce cgbfa gbadef df bfd gaecdb ebdgcf cgbde gdcfb acgbdef | fbd fd cgfba fdb
dca da bdfgc cadbef ebfacdg eadb bfeac fdcgae bfcad fgecab | adeb abcfegd gbfcea ad
gcedaf gfdecb gb caefg cebga fgacbe fabg dcabe agbfced bcg | agfb begca dcgeabf bg
dbgac cdf fagde fedbacg fgdabc acdbge dfgca ecafdb gcbf fc | bcgf cfbg ebgcdaf bdagc
ed fedag dge gceaf dfabg dcfe afdegcb aebfgc dgbaec dagecf | acgefb de afdeg gcdeba
dca fcbdg fgecab ad adbcg debfgca ecbdag gceba facged edab | acefgb ad gfebdca fcdaegb
edaf gfeab ebfgda dfbga dacgb gfbceda bgeacf df gfd bcdgfe | df fdg aegfdb feabg
bcfagde dgabfc beafc geca fgcaeb fdbec ac fca gabfe gfdaeb | gcfbead fca gebfa caf
aecbg cfeagd cadfbe dgceb cgfae edacfgb fecbag ba abc gfba | ab bca bca fbag
bdc aebgc db cdgef dagb geabcf ebgacd gdebc aedcbf gadfbce | befacdg abgced dbc bdceg
egbacfd dabgfc beagfd ef dbef dgbaf fge gcfade cgbea eafbg | dgcafe befd feg fedb
faebd cbdaf edfgab fdcbeg dfgbe ae dagfce bedgafc ead beag | dea ae abcdfge fgedbac
gfabd fagcde cdbefa gaebdf ecfdbga gaefd fab aebg cgbfd ab | ebga fdbeagc fba gadfec
bfa cfga acbfe edcfbga fa fagebd bdcfe aecfgb ebdacg geacb | gfca bgace gafceb gcaf
dgbae dcfegb bgdfc fagbcd gca dafc ca dabgfce cgbda agbecf | cafd ac cag cga
fcag egdca gc agedb cgd adefc bcfged cbefad egcfad efcagdb | cg dcg cgd cgd
ea fdeacb eab bgdefc adgcefb bfdga face cbdfe bedaf dcgaeb | gcbfdae ecdgbfa bea bae
acbe ecd feacdb ec egafd eadcf cbdaf fgcdba cfbagde efcbgd | dgcabfe ce bgfedc egdbfac
cdageb badf gbfcad fcgab fgadebc gecfd bd eacfbg dbg bgfcd | febcdag bd fadb fdba
adgcbef bef cgedba fbecag efbca baegc bf gcdbfe bfga acefd | agfb bgfa gbaf bef
bcfade agcde acd edcbg feagcd gdaf dbeacgf ad eafcgb fcega | cad gdcefab dgaf ebdgfca
df abfce degbc cfd acgfed gbdf bacfegd efdcb badceg egfcbd | cebgd bgdf bceaf df
bfdc eabdf ebcad fgead bcdeag fba bf bcedgfa cdebfa bcegaf | defba cfbd bdcf fabceg
dbaefcg ec aec dfgbea cafeg aedcbg gacedf egadf gfcba cfde | eac efcd ec egcfda
ecgfdab gcefb bcfa af cgfdbe aegbf ebadg eacgfd gbecaf afg | aedbg bafc acfb dgfbace
    ",
    )
}

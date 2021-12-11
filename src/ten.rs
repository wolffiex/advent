#![allow(dead_code)]

use std::collections::HashMap;

fn main() {
    let input = get_input();
    let split = input.split("\n");
    let scores: HashMap<char, usize> = HashMap::from([
        (')', 1),
        (']', 2),
        ('}', 3),
        ('>', 4),
    ]);

    let mut line_points: Vec<usize> = Vec::new();
    for s in split {
        let result = find_incomplete(s);
        if let Some(mut compeletion) = result {
            let mut points: usize = 0;
            let mut ss = String::new();
            for c in compeletion {
                points = points * 5 + scores.get(&c).unwrap();
                ss.push(c);
            }
            println!("{} : {}", points, ss);
            line_points.push(points)
        }
    }
    line_points.sort();
    println!("Middle: {}", line_points.get(line_points.len()/2).unwrap());
}

fn find_incomplete(line: &str) -> Option<Vec<char>> {
    let delimiters = HashMap::from([
        ('{', '}'),
        ('[', ']'),
        ('(', ')'),
        ('<', '>'),
    ]);
    let mut expected: Vec<char> = Vec::new();
    for c in line.chars() {
        if let Some(closing) = delimiters.get(&c) {
            expected.push(*closing);
        } else {
            // closing char
            let e = expected.pop();
            if e != Some(c) {
                return None;
            }
        }
    }
    expected.reverse();
    return Some(expected);
}

fn xget_input() -> &'static str {
    return "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
}

fn get_input() -> &'static str {
    return "{<<[{(<<<([[<<{}()><<>{}>]{[(){}]{{}()}}]]<[{(()())<[]{}>}(({}[])(()[]))]>)>>{{[<[[(<><>)<{}()>]{(
(({[(<{[[{{[{[<>[]][{}{}]}([{}()][<>()])][[{()()}(()<>)][([]){<>[]}]]}}{<{{{<>{}}(()<>)}[(
<{({[(<{[[((({()()}<<>()>)){<{[]()}<[]()>><[()[]]>})<{[<[]{}>]<{{}[]}{()[]}>}<[[[]<>]]<(<>())(<>{})
<{<[{[(<<(<(<[(){}]{()()}>)[{({}())[<>{}]}]>((<(()<>){<>()}>{{{}<>}([][])})))([({[{}()]<[]()
<[<{(<{{{{[{[{[]{}}<()[]>](<[]{}><[][]>)}<<<<>{}>{<>()}>{([]<>)(()[])}>]}}(<[{[<[]()>[[]<>]]}]>([{
{[<({<{{([{{{(()<>)(<>{})}}}{(([()()]{()())){[{}{}]([])})<<<()()><<>()>>((()()){[][]})>}](({{<<
<{<[[(<(((<{[[()<>]{{}()}][(<>)[<>])}>)))>({{{{((<(){}>(()<>))){(<()<>>(()()))[<()<>><[]<>>]}
<{<({(<(<{{<[((){})[()<>]]{{<>{}}([][])}>}[<{[[]()]}{<[]{}>(()<>)}>(<[()]<{}()>><{()<>}{[]()}>)]}
{<<({{{(<{{<<[{}{})[(){}]>{<<><>><[]{}>}>({({}())[<>[]]}{{{}{}}<<><>>})}}({[[(()){(){}}]({[]()}<<>{}
{{{<(([{<([[[[<><>][[]<>]](({}[])<{}{}>)][[<[][]>{{}<>}]<([]{})([]{})>]]{<[(())({}{})]<((){}){<>[]}
[(<(<<[{{{<<[({}[])<<>>][{{}<>}<[]{}>]>([({}<>){{}[]}]{(<>{})})>}(<<<{()<>}{(){}}>(([]<>){{}[]})>
{{<([({([<([[(()[])<[]()>][{{}{}}<<>()>]](({<>}{<>[]})<{[][]}{()[]}>))[[<({}[])<<>[]>>[<<>{}>({}<>)}][(<<><>>
<<<<{{(<{<(<[<<><>>(<>[])]{{<>}((){})>>(({<>()}(<>{})){{{}{}}[(){}]}))<<[[()()][[]<>]]([<><>])><[<[]<>>[[
<<[<(([(<([<({<><>}{[]<>}){(<>{})<<><>>}>({<<>[]>{()[]}}{(<>())[()[]]})])(<(<{[]<>}{[]{}}>{[()
(<[<(({{{<(<<{[][]}([][])>[<{}{}>{{}<>}]><<<{}[]>[(){}]>>){{([()<>][{}<>])[<<>()>[<>[]]]>[{<[]{}>[[]{}]}[(
<([<(<<{[{{{{{{}[]}{()<>}}([{}()]<{}()>)}{{({}())(<>())}[[<>()]<()()>]}}<{<[<>]>[[[]{}]({}{})]
{[[[[{(<({[[<{{}()}(<>{})>{<<>{}>}]][<[<[][]><[]{}>](([][]))>({<[]()>[{}[]]}[[<>{}]<[]()>])]}(<<(<<>()>)[{<><
({(([<<({<<<{([][]){<>()}}<[{}<>]{<>{}}>](<[()<>][[][]]>({()[]}[[][]]))>{[<[[]{}](()())>((<><>)<{}{}>)][
{[(([{[(<[[(<{{}[]}{()()}]<({}<>)[<><>]>)[[[<>{}](<>())][[{}{}]<{}{}>]]]{<<({}[])[<>[]]>{{
{{([<[{[<(([{({}{})({}[])}[{<>[]}[()()]]][([<>{}](<>))]))>{{([<<()()>(<><>)>((()[]))]]<<(<[]{}>[<>[]]
{{{<[[<{[(<{{({}{})<<><>>}(<[]{}>[(){}])}({{[]()}(()<>)]<{[]<>}[<>[]]>)>[{((<>()){()()})((<><>)<<>>)}
({{<<([<<[<([({}())([]<>)]{<(){}>})([[()()]])>]>><<[<(({<>{}})[(()<>){{}()}])([{()[]}[<><>]](<<>[]>[[]{}]
([<<(<<({<<{(<()()>)[<<>()>[(){}]]}({(()<>)([][])})>>}({<[{{(){}}<{}{}>}<([]{}){[][]}>]>}))[[([[[([][]){(
([[<([(<{{(({(<><>)<()>}[{()<>}({}<>)]))<([[()](()[])][{()<>}[<><>]])<<[[]{}]<()[]>><{<>[]}[{}[]]>>>][{{<<()[
{[([<{((<[{{<{<>{}}><([][])<<>[]>>}<<[{}[]]>([(){}])>>{<([()[]]{{}()})(({}[]))><<{()()}<()[]>>
<({<(<[{<[[{[{()<>}[()[]]]{[[][]]({}<>)}}<<([]())>>]{(<{<><>}<[]()>><{<>{}}>)[(<{}[]><<>()
[{({[<{<(<<([{[]<>}([]())]{{(){}}})>{(<{<>{}}(<>{})>{{[]<>}([]())})({<<><>>}({()[]}<{}<>>))}>([
<<<<<<<{<{{[[<[][]>{<>{}}]<<{}()>[[]{}]>]((([]{}))({<><>}({}<>)))}[<[<[][]>][[<>][{}[]]}>[{(()<>)}[<(){}
([[{<{(<[{[([[{}{}][()()]](<{}[]>[<>[]])}]}(<<<((){})[<>()]><{{}[]}([][])>>>(<[<{}<>>{{}()}][{()()}(<><>)]>
{[{[{([[[[<[([(){}]<{}<>>)[({}())(()[])]]>]<([<{{}<>}>[<<>>{()[]}]]{<<<>{})<<>{}>><<<>{}>(<><>)
{<([<((((<[[{<<>{}>[<>()]}]<<(()<>)<()()>>[([]{})<(){}>]>][({<{}[]>}<[()<>](<>())))<<[[]()]<
([[(<<{[{<[<<<<>[]><<>[]>>({[]<>}<<>{}>)>([<<><>>(()())]({{}()}))][[<([]<>)({})>{{[]()}[[][]]}]<([
<[<[[([[([<[([(){}]<<>{}>)<(()())[{}{}]>]{({()()}{[][]})((()[])({}{}))}>[[{[{}{}](<><>)}{{[]{}}{
[(<({<{(([{[<((){})>{{[][]}{()[]}}]}]{((<([]())<<>{}>>{{<>()}{[]}])){(<<{}<>><<>[]>>){({{}[]}<<>[]>)<[(){}]
{((((<[[{<{({([]())[{}]}[<{}[]>{()}])}>}]{<[<<(<{}{}>(()[]))>[{{<><>}{[]{}}}<[[][]][()<>]>]>](<<{[[](
<((({(<[[({({({}{}}{{}[]}})([{[]}({}[])]{[{}()]({}())})}{([{{}{}}(<>[])]<({}[])[<>()]>)[[<
(([([[([<((<(<()()>({}()))[[<>]<<>[]>])[<([]<>)>(<()()>(<>{}))])<{[{[][]}(()<>)]}[[({}[])<()
{{(((([<<(<{<[[]<>][<><>]><[{}()]<<>()>>}<{{{}()}([]{})}(<()<>><{}<>>)>>){<({[()[]](<>{})}[[<>()](()[])])<
<(<<([<[[([(([[]()]<(){}>)[{{}()}[{}[]]])][<<<<>()>(<>)>{{()[]}[{}<>]}>])([[([<>[]]<<>()>){{<>()}<()<>>}][
<<<([([{((([<{[]{}}<(){}>>[(()<>)[[]{}]]]<{<[]{}><[]{}>)>)<<[[{}{}]{{}()}][{()[]}[()()]]>([{[
{(<<{<[<{<{{(<<>()>{{}[]})[{[]<>}{()[]}]}}(({({}<>)<()>}<[{}{}]{{}<>]>)({<{}{}>((){})}{{[][]}[[][]]
{([[<[<[[(<([<()>{[]{}}]<[<>{}](()[])>){({{}}([][]))}>)[({(((){})<{}{}>)})[({({}[])[[]()]}<<{}<>>(<><>)>)
[[<[[([([[{((<{}{}><<>()>)<<<>{}>((){}>>)<<<(){}><[]{}>>([[]{}])>}({[[<>{}]({}())]<<<><>>>})]]((<{{(
[<{{[([{[[<((([]){(){}})<{<>[]}{[]{}}>){[[[]{}]{[]{}}]}>]{[([<()[]>[<>{}]](([][]){<>{}}))]}]}{
({([{((<[([[<[{}()][()[]}>]{<{<>[]}<{}{}>>[<<>>{()<>}]}]<({{{}<>}{{}<>}}{<<>{}><()<>>})<(([]()
({<(({{{{{<{{{[]()}<[]>}[{<>{}}[[][]]]}[{({}())({}())}{({}{})<[]{}>}]>}{[<(([]())({}<>))>[({{
<<<{[[[<<{[<<((){})(()())>[[[]<>]<<><>>]>{{[{}{}]{(){}}}<{<><>](()())>}][(<{(){}}>)]}>(<[<[
{[<[{<({([({(<<><>>){([][]){(){}}}}[{{()()}[<><>]}]]{<{<<>>(<><>)}{{<>[]}(<>)}>[{{[]<>}[<>[]]}((<>[]){[]})]}]
<{[{{[<{[<<<[({}<>)[{}]][{{}{}}]>(({()[]}<{}<>})<[{}]([][])>)>>[[<<[()<>]({}[])>(<[][]>[{}()])>({{
(({[<([({{<{{(()<>)[{}[]]}<{(){}}<<>()>>}(<[()()](<><>)>{[()()]([]<>)})>({(([]()){{}})<[[]()](<>{
{{<<[<((<<({[(()<>)([]<>))({[]{}})}[([()[]]{[]{}})])<((<[]{}>(()())))>>{<<<<{}>(<><>)>>(<{<><>}>)>}>
{{([{{[{([({[<{}<>>({}[])][({})<()<>>]}[<(<>())(<>{})>({(){}}<[]()>)])<{<<{}{}>>[<()()>(<>{})]}({[()<>
(<(({{<<(<[{{{(){}}(<>[])}<(<>())>}{<{<><>}({}{})>{<[]{}><()[]>}}][<((()[])<{}<>>)((<><>){()()})>]
<((({{{([([(<{()[]}[{}()]>)][[[[{}[]][<>{}]][{{}}[()()]]]([[{}<>]]{(<>())(()[])})])[{{(<{}()]<<>()>)[{<><>}[
(([({[([({([[[[]{}]({})]{{[]{}}{{}()}}][[[<>{}]]<<()[]>[<>{}]>]){<<[[]()]<{}[]>>({(){}}<<>{}>)>
(<{<[({(<({{{[[]<>][<>()]}}[{[[]()]{[]{}}}]}<(([[]()])<<<>[]>{[][]}>)[<([][]){<>{}}>([{}{}]{{}<>
<{((([[{{{{[[{[]()}[<><>]]<[<><>][{}{}]>](<{{}}({}{})>{<{}>[()<>]})}{([<[][]>]({<>{}}<[][]>))<{[<>{}]}>}
{<{<([<<[<<{<<<>{}>[()()]>{<[]()><()[]>}}[{([]<>>{[][]}}[({}[])([][])]]>{<[[<>[]]<{}[]>]<[{}[]][[][]]>>}>{{<
([<[[[<[[<[{<<<>{}>[<>]>(([][]))}([<{}[]>{<>{}}][<{}[]>])]<{{(()[])[[]()]}}[{{{}[]}[{}<>]}(({}(
[{<[{[<{<{(({{{}()}<[]{}>}{[[]()]})<{[<>[]][<><>]}<(()<>){[][]}>>)<(({{}()}<{}>){(()){[][]}})<{[()<>
(<{([[{{(((([(()<>)](({}[])[()[]]))])([(([<>]<<>{}>)<{(){}}{<><>}>)[[{[]{}}<<><>>][({})[<>[]]]]]{{<({}{})<()
{<{(<<<<{{{{[<()()>{(){}}](({})<()()>)}{<[[]{}]<<><>>>}}[<{[<>]{()<>}}[[[]()]]>]}({([[<>[]]{{}()}]){<<{}<>
([[{<[[{[<{[<({})<<>[]>>{{[]<>}[[]{}]}]{([<>[]][[]{}])(<{}><()>)}}{<{{{}[]}<{}{}>}{<<><>>[[]()
((<[<(({<{{{{<()<>>}((()())(<>{}))}}{[{({}[]){<>()}}[<()<>>]}}}[<[({()<>}{<>{}}){{<>()}[<><>]}]<(<(){}>(
[({[(([({[({(<<>[]>)([{}<>]{()()})}<<(<>{})(()[])><[<>{}](()<>)>>)]<[[{(<>()){<>()}}{(<>()}{{}[]}}]]>}{[(([
[{{<<(({<<<[[({}()){[]}][{(){}}]]>[[<{()<>}([]())>]]}((<<(<>())<()[]>>[(<>()){[][]}]>{[({})<()()>]}))
[{((([{<<<[<<[{}]<[]()>><([][])[<>{}]>>{[<{}<>>{[]<>}]([<>[]])}](<(<{}[]>{{}()})>([(<>{})<[]<>>]{{<>[]}{<>
(<[([<[[{{[(<(()())(<>{})><<{}[]>{<>[]}>)<<([]{}){[]}>{(()[])[<>]}>]({<{<>()}>[<<><>>[()[]]]}[(
[<(<[([<[([<[[[]][()()]]{[[][]]{[][]}}>(<([]{})>)][<({[][]})<({}[])([]())>><{{[]()}}{<{}[]>)>])[{[<<()(
{{<{[(({[[(((<<>()><<>})<(()()){[]()}>)(<{[][]}<[][]>>{[[][]]{[]}}))<[<{<>{}}>]>]{{[[{<>{}}[
<{<[{{[<<[[{<{{}()}><[()()]([]{})>}][<[(()<>){<><>}]{[{}()]}>{[{()[]}[<>{}]]{(()[])[<>[]]}}]]<([{{<>()}{
{<<[<[(<{([<{[<>{}][{}{}]}({[]()}(()))>(<([]{})><<<>{}><{}<>>>)][{{{[]{}}}[([]<>){[]{}}]}<(<<>
{[(<{({(<<({(({}()){[]{}})<(()[])>}[((<>{})([]()))<{{}[]}[<>[]]>])<[<[{}()]{[][]}><<[]{}>>]>>{
({(((<<{([{[[<<>{}>[[]()]][{<>[]}<()[]>]]}{({<[][]}{<>()}})({<{}<>>{{}<>}}<{[]{}}({}())>)}]
[[{[{<<<[[([<<<><>>[()[]]><{<>[]}>])]{([[<<><>>[<><>]][{<>()}{{}{}}]]([{<>()}<{}[]>]{{[][]}([]<>)})){{(
{<([[[[[[{([[<[][]><{}()>]<([][])<(){}>>]({[[][]]{()[]}})){({<{}{}><<><>>}){([{}{}]{<><>}){(
({<[(((((<(<[[[]<>](<><>)]<{<>[]}[(){}]>>>(({[<>[]]<<>()>}{[()()]<{}<>>})[{<{}[]>[{}[]]}[[[](
({{[(({({[({{<<>{}>((){})}(({}{}))})]<[<[({}{})((){})]<<<>{}>>><{[{}[]](<>{})}<<[]<>>([][]
({([[<{(([<(((())[<>()])[[[]]<{}()>])<[<()<>>{(){}}][{()<>}[[]<>]]>><[<[<>{}]{<>()}>](<(<>())
{((<<[({(((<{{{}{}}<<>{}>}<({}())<<>[]>>><([<>])<[[]()]>>)[[<<<>[]>(()<>)][([]())]]{{<[]<>>([]{})}<{{}()}(
((([{{[(<{{((<{}{}><<><>])[({}<>)[[]<>]]){<{()[]}[[]()]>(([])[{}[]])}}(<[[[]{}][{}[]]][[()[
[{{<{<(<[<{({<[]<>>}<<[][]>[()[]]>){<(<>()]{<>[]}>[<{}[]>{()()}]}}>{[[{[()()]<<>[]>}<[[]<>](<>
<((<[<<[(<{[[(<>{})(<>{})]<{{}()}>](([{}{}][(){}]}[[<><>]{{}<>}])}<{[[{}{}][{}()]][(()())<[][]>]}>><<
[(({[[{[{<{[(<<>[]><{}{}>)(([]{}){(){}})]{[[[][]]<<>()>]({<>()}(<>{}))}}[<{{<>()}([]{})}[{[]()}[()[]]]>[[
[({[([{[({<<([<>[]])(<{}<>><{}{}>)>>{<[{[][]}<{}[]>){[<><>]<()()>}>[(<<>{}>{[][]})[{()<>}(<
[<<<{[[({<{[{<()()><{}[]>}]<{(()())<{}{}>}{{<>{}}<<>()>}>}<{[{{}[]}(<>)]((<>())[()<>])}{<[<
[[(<(({([<<<[<{}<>>(<>{})][{<>[]}([][])]><<<[]<>>(())>[[(){}][()<>]]>><{(<{}<>>[{}{}])<{()
[<<{[([<([([(({}{})[<>[]])<{[]<>}(()())>]<<<{}[]>{<>{}}>{({}[]){{}<>}}>)(([[[][]]](<{}{}><[]{}>)){[<{}[]>{{}<
[<{[({[{{<[([<<><>>[[][]]]([{}{}]({}{})))([[{}<>]<[]{}>][[<>()]{[][]}])]<<(<<>{}><<>{}>){[<>{}](<>[
({[<[[{[(<<[<{[][]}([]<>)>[<<>[]>[()[]]]]<(<{}[]><<>{}>)[<[][]>]>>[[{{<><>}{<>}}<[{}()]{<><>}}]{(<<>{}>
<<({[<[([{(([{(){}}][(()<>){{}<>>])<(<<>>)<[[]{}]{<>{}}>>)}<[<{<<>()>(<>())}>[{<[][]>(<>{})}[<{}[
{(<(({(((({({[<><>]<[]()>}{{<>[]}{<>()}}){({{}()}{()()})((<><>><()[]>)}})<[[{({}())({}{})}[{<><>
({(<({(<{((({([]{}){[]()}}({()<>}{(){}))){({<>[]}<[]{}>)<<[]()><[][]>>}))}>){(({[[[<<>[]>][<()[]>([][])]](
([<{<{(([(<<([{}]([]{}))<(()())({})>>([<{}()>(<>())])>{[[[()()]]<[<>{}]<{}{}>>][<{(){}]{<>{}}><(<>[
[<[{(({<({([[{[][]}({}[])]([{}()]<{}[]>)])((<<[]{})({}())>{(<>{}){{}()}})[[<<>>(())]])}[{[[[()()](<>[
(<(((({(<{[{(<<>()>([]()))[<[]()>[<>]]}]<<[[()<>]([]{})]><(<[]{}>{<>[]}){{[]()}(()[])}>>}>)
<(<<(<<[(([{<<[]{}>(()())>{[(){}]([][])}}][(<{{}{}}{[][]}>(([][])<<>[]>))]))[[<<([{}[]]<[]{}>)>[([()[]]<{}
[<<<<(<(((<[(<()[]><[]{}>)](<{[]{}}[[]{}]>[<{}<>><<>{}>])>[[{{<>()}}([()<>][[]{}])]<(<[][]>((){}))<<[]
[<<[([{([[({{{()()}({}{})}({()[]}{<>{}})}{[{<>[]}[<>()]]((<><>)({}()))}){[{<{}{}>(<>[])}{{{}<>}}][<([]{})
[((({<<<<[<([{(){}}(()[])}{{[][]}({}{})})([{{}{}}{[]()}](([]<>){<><>}))><<<<{}[]>(<>[])>{(<>{})[<>
{<[({<[<[([{<{[][]}<{}{}>>([{}>)}([{[][]}({}[])][[{}()]])]{[[[{}<>]<<>{}>]{<<><>><[][]>}][(
{({<{[<{[<<[(<<>><<><>>)[<()[]>[{}[]]]]<[(<>[])<{}>]{({}[]){()[]}}>>{[<{[]()}({})>((())[()[]])](<({}<>)";
}
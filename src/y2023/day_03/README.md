source: [https://adventofcode.com/2023/day/3](https://adventofcode.com/2023/day/3)

## --- Day 3: Gear Ratios ---
You and the Elf eventually reach a [gondola lift](https://en.wikipedia.org/wiki/Gondola_lift) station; he says the gondola lift will take you up <em>to the water source<em>, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can <em>add up all the part numbers<em> in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently <em>any number adjacent to a symbol<em>, even diagonally, is a "part number" and should be included in your sum. (Periods (<code>.</code>) do not count as a symbol.)

Here is an example engine schematic:

<code>467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
</code>

In this schematic, two numbers are <em>not<em> part numbers because they are not adjacent to a symbol: <code>114</code> (top right) and <code>58</code> (middle right). Every other number is adjacent to a symbol and so <em>is<em> a part number; their sum is <code>4361</code>.

Of course, the actual engine schematic is much larger. <em>What is the sum of all of the part numbers in the engine schematic?<em>

## --- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A <em>gear<em> is any <code>*</code> symbol that is adjacent to <em>exactly two part numbers<em>. Its <em>gear ratio<em> is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

<code>
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
</code>

In this schematic, there are <em>two<em> gears. The first is in the top left; it has part numbers <code>467</code> and <code>35</code>, so its gear ratio is <code>16345</code>. The second gear is in the lower right; its gear ratio is <code>451490</code>. (The <code>*</code> adjacent to <code>617</code> is <em>not<em> a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces <code>467835</code>.

<em>What is the sum of all of the gear ratios in your engine schematic?<em>
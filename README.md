# Wordle Optimality Analysis

This project is an analysis of the massively popular word game Wordle, and this code mines for 20 of the most mathematically optimal starting words in wordle, and performing a kind of optimality measure on each of them. The attributes to each word is their minimum decision tree size needed to test combinations, the average number of guesses the code can solve the Wordle in starting with the starter word, and the maximum number of guesses that the code needed in order to find the word. Everything is with compeltely optimized gameplay, and because of this, the solver code needs to sometimes omit information that has been given from the previous word in order to maximize the amount of new information available within the next word. Because of this, the code is not completely aligned and fit to mimic human gameplay; for that there is some better code examples out there that also come with a "Hard Mode", where words that are highlighted either yellow or green must be reused again, mimicing normal human gameplay for this game. 

Also, I shall note that as for the total decision tree sizes, not all of them are optimal. For instance, the word "trace" has been proven to have a minimal decision tree size of 7926, while this code is just minorly unoptimal with 7927 for its size. For use in data visualization programs, I have also included the results in the table below within a CSV file.

<div class="row"><div class="col-md-12"><div class="panel panel-success"><div class="panel-heading "><h3 class="panel-title">Optimalities</h3></div>
<table border=1 class="table table-striped table-bordered table-hover table-condensed">
<thead><tr><th title="Field #1">word</th>
<th title="Field #2">total_decision_tree_size</th>
<th title="Field #3">average_guess_number</th>
<th title="Field #4">maximum_guesses</th>
</tr></thead>
<tbody><tr><td>trace</td>
<td align="right">7927</td>
<td align="right">3.4241900</td>
<td align="right">5</td>
</tr>
<tr><td>crate</td>
<td align="right">7927</td>
<td align="right">3.4241900</td>
<td align="right">5</td>
</tr>
<tr><td>salet</td>
<td align="right">7920</td>
<td align="right">3.4211664</td>
<td align="right">5</td>
</tr>
<tr><td>slate</td>
<td align="right">7928</td>
<td align="right">3.4246220</td>
<td align="right">6</td>
</tr>
<tr><td>reast</td>
<td align="right">7925</td>
<td align="right">3.4233260</td>
<td align="right">5</td>
</tr>
<tr><td>parse</td>
<td align="right">7997</td>
<td align="right">3.4544277</td>
<td align="right">6</td>
</tr>
<tr><td>carte</td>
<td align="right">7950</td>
<td align="right">3.4341252</td>
<td align="right">6</td>
</tr>
<tr><td>caret</td>
<td align="right">7965</td>
<td align="right">3.4406047</td>
<td align="right">5</td>
</tr>
<tr><td>peart</td>
<td align="right">7981</td>
<td align="right">3.4475162</td>
<td align="right">6</td>
</tr>
<tr><td>carle</td>
<td align="right">7937</td>
<td align="right">3.4285097</td>
<td align="right">6</td>
</tr>
<tr><td>crane</td>
<td align="right">7934</td>
<td align="right">3.4272140</td>
<td align="right">6</td>
</tr>
<tr><td>stale</td>
<td align="right">7968</td>
<td align="right">3.4419007</td>
<td align="right">6</td>
</tr>
<tr><td>earst</td>
<td align="right">7975</td>
<td align="right">3.4449244</td>
<td align="right">6</td>
</tr>
<tr><td>heart</td>
<td align="right">8033</td>
<td align="right">3.4699783</td>
<td align="right">6</td>
</tr>
<tr><td>reist</td>
<td align="right">7987</td>
<td align="right">3.4501080</td>
<td align="right">6</td>
</tr>
<tr><td>least</td>
<td align="right">7955</td>
<td align="right">3.4362850</td>
<td align="right">6</td>
</tr>
<tr><td>react</td>
<td align="right">7982</td>
<td align="right">3.4479482</td>
<td align="right">5</td>
</tr>
<tr><td>cater</td>
<td align="right">8010</td>
<td align="right">3.4600432</td>
<td align="right">5</td>
</tr>
<tr><td>carse</td>
<td align="right">7970</td>
<td align="right">3.4427645</td>
<td align="right">6</td>
</tr>
<tr><td>tried</td>
<td align="right">8003</td>
<td align="right">3.4570193</td>
<td align="right">5</td>
</tr>
</tbody></table>
</div></div></div>
-----
As this code can verify, the most optimal word was "Salet", which has the following stats below:
<div class="row"><div class="col-md-12"><div class="panel panel-success"><div class="panel-heading "><h3 class="panel-title">Stats for Salet</h3></div>
<table border=1 class="table table-striped table-bordered table-hover table-condensed">
<thead><tr><th title="Field #1">salet</th>
<th title="Field #2">total guesses: 7920</th>
<th title="Field #3">avg guesses: 3.4211664</th>
<th title="Field #4">max guesses: 5</th>
</tr></thead>
<tbody></tbody></table>
</div></div></div>

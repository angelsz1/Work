<h1>Work</h1>
<p>Work is a CLI tool for using a pomodoro timer, written in Rust. With this tool, you can easily manage your work and break times to stay productive and focused.</p>
<h2>Features</h2>
<ul>
  <li>Allows you to set the number of rounds with the `-r` flag</li>
  <li>Allows you to set the work time in minutes with the `-w` flag</li>
  <li>Allows you to set the relax time in minutes with the `-x` flag</li>
</ul>

<h2>Usage</h2>
<p>To use Work, you need to have <a href="https://www.rust-lang.org/tools/install">Rust</a> installed on your computer.</p>

<p>Clone the repository to your local machine:</p>
<pre>
$ git clone https://github.com/angelsz1/work.git
</pre>
<p>Build the project using the following command inside the project root:</p>
<pre>
$ cargo install --path .
</pre>


<p>Run Work with the following command:</p>
<pre>
$ work -r 5 -w 25 -x 5
</pre>
<p>This will start the Pomodoro timer with 5 rounds, each round lasting 25 minutes of work and 5 minutes of break.</p>

<p>Alternatively, you can run Work without any flags, which will use the default values of 1 rounds, 45 minutes of work, and 15 minutes of break:</p>
<pre>
$ work
</pre>

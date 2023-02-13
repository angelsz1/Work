<h1>Work</h1>
This is a CLI tool for using a pomodoro timer, written in Rust. With this tool, you can easily manage your work and break times to stay productive and focused.

<h3>Features</h3>
Allows you to set the number of rounds with the -r flag
Allows you to set the work time in minutes with the -w flag
Allows you to set the relax time in minutes with the -x flag
Usage
To use the Pomodoro CLI, you need to have <a href="https://www.rust-lang.org/tools/install">Rust</a> installed on your computer.

Clone the repository to your local machine:

  $ git clone https://github.com/angelsz1/work.git
  
Build the project using the following command:

  $ cargo install --path .

<h3>Example</h3> 
<p style="color:MediumSeaGreen"> work -r 5 -w 25 -x 5</p>
This will start the Pomodoro timer with 5 rounds, each round lasting 25 minutes of work and 5 minutes of break.


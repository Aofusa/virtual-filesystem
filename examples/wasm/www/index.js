import { Cli } from "virtual-filesystem-wasm";

const cli = Cli.new();

const commandText = document.getElementById("virtual-filesystem-wasm-command-input");
const runButton = document.getElementById("virtual-filesystem-wasm-command-run");
const commandOutput = document.getElementById("virtual-filesystem-wasm-command-response");

runButton.addEventListener("click", event => {
    const command = commandText.value;
    commandOutput.innerHTML = cli.run(command);
});


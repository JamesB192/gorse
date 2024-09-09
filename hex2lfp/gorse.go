package main

import (
	"fmt";
	"io";
)

func main() {
	fmt.Println("Please enter a hex encoded LFP.")
	fmt.Println("Such an argument is two 8 digit hexadecimal numbers.")
	fmt.Println("The numbers may be joined with a decimal or directly.")
	fmt.Println("There may be a number of space/tabs and/or a 0x prepended.")
	fmt.Println("There may be a number of space/tabs apended.")
	fmt.Println("Press <control>+d to exit.")
	for {
		hex := string("")
		_, err := fmt.Scanln(&hex)
		if err != nil {
			if err == io.EOF {
				break
			}
			fmt.Printf("Error: %v\n", err)
		}
/*		if bufio.EOF == got {
			fmt.Print("Last one\n")
			break
		}
*/
		success, lfp := HexToLFP(hex)
		if false == success {
			fmt.Println("That seems to be formatted wrong")
			continue
		}
		fmt.Printf("%016x -- %d\n", lfp, lfp)
	}
}

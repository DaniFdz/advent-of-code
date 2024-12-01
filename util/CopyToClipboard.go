package util

import (
	"bytes"
	"fmt"
	"os/exec"
  "runtime"
)

func CopyToClipboard(text string) error {
  var copyCommand string
  switch os := runtime.GOOS; os {
  case "linux":
    if IsWSL() {
      copyCommand = "clip.exe"
    } else {
      copyCommand = "xclip -sel clip"
    }
  case "darwin":
    copyCommand = "pbcopy"
  case "windows":
    copyCommand = "clip.exe"
  default:
    return fmt.Errorf("os %s not supported for copying", os)
  }
	command := exec.Command(copyCommand)
	command.Stdin = bytes.NewReader([]byte(text))

	if err := command.Start(); err != nil {
		return fmt.Errorf("error starting %s command: %w", copyCommand, err)
	}

	err := command.Wait()
	if err != nil {
		return fmt.Errorf("error running %s %w", copyCommand, err)
	}

	return nil
}

func IsWSL() bool {
	cmd := exec.Command("systemd-detect-virt")
	var out bytes.Buffer
	cmd.Stdout = &out
	err := cmd.Run()

	if err != nil {
		fmt.Println("Error running systemd-detect-virt:", err)
		return false
	}

	output := out.String()
	return output == "wsl\n" 
}

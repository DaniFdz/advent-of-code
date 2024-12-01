package util

import (
	"bytes"
	"fmt"
	"os/exec"
  "runtime"
)

// CopyToClipboard is for macOS and linux
func CopyToClipboard(text string) error {
  var copyCommand string
  switch os := runtime.GOOS; os {
  case "linux":
    copyCommand = "xclip -sel clip"
  case "darwin":
    copyCommand = "pbcopy"
  default:
    return fmt.Errorf("os %s not supported for copying", os)
  }
	command := exec.Command("pbcopy")
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

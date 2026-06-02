# ZPL Template Examples

This directory contains example ZPL templates that demonstrate the variable system.

## Template Variable Syntax

Variables in templates use the following format:
```
{{VARIABLE_NAME}}
{{VARIABLE_NAME:Display Label}}
```

- Variable names must be UPPERCASE with underscores
- Optional display label after colon will be shown in the UI
- Without a display label, the variable name will be shown as-is

## Example Templates

### simple_name_tag.zpl
A basic name tag template with three variables:
- NAME: Your Name
- TITLE: Job Title
- COMPANY: Company Name

## Using Templates

1. Open the ZPL Printer Tool
2. Click "Load Template" at the top
3. Select a .zpl file
4. Fill in the detected variables (Use mode)
5. Preview updates automatically as you type
6. Select your Zebra printer from the dropdown
7. Click "Print Label"

## Creating Your Own Templates

You can create templates in two ways:

1. **Edit Mode**: Write ZPL directly in the application and save it
2. **Text Editor**: Create a .zpl file with your ZPL code and variable placeholders

Tips:
- Test your ZPL commands first to ensure proper positioning
- Use descriptive display labels for better user experience
- Keep variable names consistent and meaningful

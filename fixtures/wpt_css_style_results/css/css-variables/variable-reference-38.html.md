# css/css-variables/variable-reference-38.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-reference-38.html"
}
```

## style[0]

```css

p { padding-left: 1em; }
#a { --style: solid; --left: black dotted; border-style: var(--style); border-left: var(--left); border-top: none; border-right: none; border-bottom: none; }
#b { --style: solid; --left: black dotted; border-left: var(--left); border-style: var(--style); border-top: none; border-right: none; border-bottom: none; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border-style”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

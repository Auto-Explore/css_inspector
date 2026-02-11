# css/css-display/display-contents-before-after-003.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-before-after-003.html"
}
```

## style[0]

```css

    /* Disable kerning because kerning may differ for different node tree. */
    html { font-kerning: none; font-feature-settings: "kern" off; }
    .flex { display: inline-flex; flex-direction: column }
    .flex::before { display: contents; content: "A" }
    .flex::after { display: contents; content: "S" }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-feature-settings”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

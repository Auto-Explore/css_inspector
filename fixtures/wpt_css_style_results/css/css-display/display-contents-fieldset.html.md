# css/css-display/display-contents-fieldset.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-contents-fieldset.html"
}
```

## style[0]

```css

  /* Disable kerning because kerning may differ for different node tree. */
  html { font-kerning: none; font-feature-settings: "kern" off; }
  fieldset, legend {
    all: initial;
    border: 10px solid red;
    display: contents;
  }
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

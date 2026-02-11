# css/css-logical/logical-values-float-clear-4.html

```json
{
  "format_version": 3,
  "file": "css/css-logical/logical-values-float-clear-4.html"
}
```

## style[0]

```css

html { writing-mode: vertical-rl; }
body > div { height: 15em; width: 10em; margin: 1em; padding: 2px; border: 1px solid silver; }
div > div { margin: .5em; padding: .5em; background: yellow; }
.a { clear: inline-start; }
.b { clear: inline-end; }
.is { float: inline-start; width: 4em; }
.ie { float: inline-end; width: 2em; }
.ltr { direction: ltr; }
.rtl { direction: rtl; }
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “float”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “float”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

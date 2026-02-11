# css/css-text/letter-spacing/letter-spacing-nesting-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-text/letter-spacing/letter-spacing-nesting-003.xht"
}
```

## style[0]

```css

      .test, .control {
          font: 50px/1 monospace;
          font-kerning: none;
      }
      .control {
          position: absolute;
          color: red;
          z-index: -1;
          white-space: pre;
      }
      .control span {
          /* to ensure both background and text overlay correctly */
          background: linear-gradient(to bottom, red 50%, green 50%, green 100%);
      }
      .test span {
          background: linear-gradient(to bottom, green 50%, transparent 50%, transparent 100%);
      }
      .wide {
          letter-spacing: 2ch;
      }
      .narrow {
          letter-spacing: 1ch;
      }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

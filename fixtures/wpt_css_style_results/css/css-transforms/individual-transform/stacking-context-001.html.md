# css/css-transforms/individual-transform/stacking-context-001.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/individual-transform/stacking-context-001.html"
}
```

## style[0]

```css

      #back {
        height: 100px;
        width: 100px;
        position: fixed;
        background: green;
        margin-top: 50px;
      }
      @keyframes scale {
        from, to { scale: 1; }
      }
      #test {
        width: 100px;
        height: 100px;
        background: blue;
        animation: scale 100s 100s;
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

# css/css-writing-modes/writing-mode-vertical-rl-003.htm

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/writing-mode-vertical-rl-003.htm"
}
```

## style[0]

```css

        div {
            writing-mode: vertical-rl;
        }

        div p {
            background: yellow;
            color: blue;
            font: 20px/1 Ahem;
            height: 6em;
            margin: 10px;
            white-space: pre;
            width: 6em;
        }

        div p:nth-child(2) {
            background: pink;
        }

        p#test {
            writing-mode: horizontal-tb;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

# css/css-flexbox/flexbox-with-pseudo-elements-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-with-pseudo-elements-001.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      align-items: flex-end;
      justify-content: space-between;
      height: 50px;
      width: 300px;
      margin-bottom: 2px;
      background: lightgray;
    }
    div.withBefore::before {
      align-self: center;
      content: 'b';
      background: yellow;
    }
    div.withAfter::after {
      align-self: center;
      content: 'a';
      background: lightblue;
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

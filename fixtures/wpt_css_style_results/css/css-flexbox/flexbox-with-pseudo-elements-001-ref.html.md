# css/css-flexbox/flexbox-with-pseudo-elements-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-with-pseudo-elements-001-ref.html"
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
    .fakeBefore {
      align-self: center;
      content: 'b';
      background: yellow;
    }
    .fakeAfter {
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

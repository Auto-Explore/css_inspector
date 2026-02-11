# css/css-flexbox/flexbox-with-pseudo-elements-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-with-pseudo-elements-002-ref.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      margin-bottom: 2px;
      background: lightgray;
    }
    .fakeBefore {
      background: yellow;
      /* This 'order' value should place us after the other elements, visually,
         even though we're ::before. */
      order: 1;
    }
    .fakeAfter {
      background: lightblue;
      order: -1;
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

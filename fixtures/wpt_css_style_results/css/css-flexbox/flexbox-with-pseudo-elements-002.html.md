# css/css-flexbox/flexbox-with-pseudo-elements-002.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-with-pseudo-elements-002.html"
}
```

## style[0]

```css

    .flexContainer {
      display: flex;
      margin-bottom: 2px;
      background: lightgray;
    }
    .withBefore::before {
      content: 'b';
      background: yellow;
      /* This 'order' value should place us after the other elements, visually,
         even though we're ::before. */
      order: 1;
    }
    .withAfter::after {
      content: 'a';
      background: lightblue;
      /* This 'order' value should place us before the other elements, visually,
         even though we're ::after. */
      order: -1;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

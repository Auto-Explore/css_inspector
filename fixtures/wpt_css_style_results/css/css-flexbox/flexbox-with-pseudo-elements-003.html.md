# css/css-flexbox/flexbox-with-pseudo-elements-003.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-with-pseudo-elements-003.html"
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
      display: table-row;
      content: 'b';
      background: yellow;
      /* If these "align-self" & "order" properties impact the rendering (as
         they should), that verifies we're being treated as a flex item. */
      align-self: center;
      order: 1;
    }
    div.withAfter::after {
      display: table-cell;
      content: 'a';
      background: lightblue;
      /* If these "align-self" & "order" properties impact the rendering (as
         they should), that verifies we're being treated as a flex item. */
      align-self: center;
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

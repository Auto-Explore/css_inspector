# css/css-flexbox/flexbox-table-fixup-001.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-table-fixup-001.xhtml"
}
```

## style[0]

```css

      div.flexbox {
        border: 1px dashed blue;
        width: 200px;
        display: flex;
        justify-content: space-around;
      }

      td {
        /* Remove any default padding for td elements, so we can compare them
           easily against blocks in the reference case. */
        padding: 0px;
      }

      .a {
        background: lightgreen;
        width: 48px;
      }

      .b {
        background: yellow;
        width: 48px;
      }

      .c {
        background: pink;
        width: 48px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

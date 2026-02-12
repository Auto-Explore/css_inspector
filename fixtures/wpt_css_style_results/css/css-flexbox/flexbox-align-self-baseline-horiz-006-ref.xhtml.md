# css/css-flexbox/flexbox-align-self-baseline-horiz-006-ref.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-align-self-baseline-horiz-006-ref.xhtml"
}
```

## style[0]

```css

      .container,
      .containerEndAlign {
        border: 1px dashed blue;
        font: 14px sans-serif;
        height: 50px;
      }

      .container > div,
      .wrapper > div{
        display: inline-block;
      }

      .containerEndAlign {
        display: flex;
        align-items: end;
      }
      .wrapper {
        /* This accounts for the 'margin-bottom: 3px' on the 'offset' element
           in the last-baseline-aligned part of the testcase: */
        margin-bottom: 3px;
      }

      .ortho  {
        writing-mode: vertical-rl;
        width: 17px;
        height: 40px;
      }

      .lime   { background: lime;   }
      .yellow { background: yellow; }
      .orange { background: orange; }
      .pink   { background: pink;   }
   
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

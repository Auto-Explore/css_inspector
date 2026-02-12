# css/css-flexbox/flexbox-mbp-horiz-002v.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-mbp-horiz-002v.xhtml"
}
```

## style[0]

```css

      div { height: 100px; border: 0; }
      div.flexbox {
        width: 200px;
        font-size: 10px;
        display: flex;
      }
      div.a {
        flex: 1 0 9px;
        background: lightgreen;
        margin-left: 1px;
        margin-right: 3px;
        border-style: dotted;
        border-left-width: 2px;
        border-right-width: 4px;
        padding-left: 5px;
        padding-right: 6px;
        writing-mode: vertical-lr;
      }
      div.b {
        flex: 2 0 1px;
        background: yellow;
        margin-left: 2px;
        margin-right: 4px;
        border-style: dashed;
        border-left-width: 7px;
        border-right-width: 3px;
        padding-left: 1px;
        padding-right: 2px;
        writing-mode: vertical-rl;
      }
      div.c {
        flex: 3 0 40px;
        background: orange;
        writing-mode: sideways-lr;
      }
      div.flexNone {
        flex: none;
        background: pink;
        writing-mode: vertical-lr;
      }
      div.flexBasis {
        flex: 0 0 20px;
        background: gray;
        writing-mode: sideways-rl;
      }
      div.spacer {
        width: 15px;
        height: 15px;
        background: purple;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

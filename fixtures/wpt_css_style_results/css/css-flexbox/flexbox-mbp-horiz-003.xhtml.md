# css/css-flexbox/flexbox-mbp-horiz-003.xhtml

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexbox-mbp-horiz-003.xhtml"
}
```

## style[0]

```css

      div { height: 20px; border: 0; }
      div.flexbox {
        width: 200px;
        display: flex;
        margin-bottom: 2px;
      }

      <!-- customizations for flex container border/padding -->
      .borderA {
        border-style: dashed;
        border-color: purple;
        border-top-width: 6px;
        border-right-width: 4px;
        border-bottom-width: 2px;
        border-left-width: 8px;
      }

      .borderB {
        border-style: dashed;
        border-color: purple;
        border-top-width: 4px;
        border-right-width: 5px;
        border-bottom-width: 6px;
        border-left-width: 7px;
      }

      .paddingA {
        padding: 4px 3px 2px 1px;
      }

      .paddingB {
        padding: 8px 11px 14px 17px;
      }

      div.child1 {
        flex: 1 0 24px;
        background: lightgreen;
        border-style: dotted;
        border-left-width: 2px;
        border-right-width: 4px;
      }
      div.child2 {
        flex: 2 0 10px;
        background: yellow;
        border-style: dashed;
        border-left-width: 7px;
        border-right-width: 3px;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

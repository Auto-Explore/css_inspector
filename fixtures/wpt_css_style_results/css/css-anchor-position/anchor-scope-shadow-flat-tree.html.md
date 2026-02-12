# css/css-anchor-position/anchor-scope-shadow-flat-tree.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scope-shadow-flat-tree.html"
}
```

## style[0]

```css

      ::slotted(.outer_anchored), .inner_anchored {
        background: coral;
        position: absolute;
        top: anchor(bottom, 1px);
        position-anchor: --a;
        width: 5px;
        height: 5px;
      }
      .anchor {
        background: skyblue;
        height: 10px;
        anchor-name: --a;
      }
      .cb {
        position: relative;
        width: 200px;
        height: 200px;
        border: 1px solid black;
      }
      .scope {
        anchor-scope: --a;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

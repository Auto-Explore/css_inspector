# css/css-view-transitions/scoped/display-change-during-transition-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/scoped/display-change-during-transition-ref.html"
}
```

## style[0]

```css

    .box {
      position: absolute;
      z-index: 0;
      will-change: transform;
      contain: strict;
    }
    #scope {
      position: absolute;
      background: #eee;
      left: 40px;
      top: 40px;
      width: 490px;
      height: 190px;
    }
    .part {
      left: 30px;
      top: 30px;
      width: 120px;
      height: 120px;
      background-color: purple;
      transform: translateX(150px);
      outline: 5px solid green;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

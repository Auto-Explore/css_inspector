# css/css-position/position-absolute-crash-chrome-007.html

```json
{
  "format_version": 3,
  "file": "css/css-position/position-absolute-crash-chrome-007.html"
}
```

## style[0]

```css

  body {
    overflow: hidden;
    margin: 0px;
    font-size: 24px;
  }
  #block-container {
    position: relative;
  }
  #css-container {
    position: relative;
    font-size: 12px;
  }
  #anonymous-parent {
    background-color: #FFFF7F;
  }
  #anonymous-split {
    background-color: #FFD997;
  }
  #css-container {
    background-color: #BEE0FF;
  }
  #abs {
    background-color: rgba(0, 255, 0, 0.5);
    position:absolute;
    top: 0px;
    left: 0px;
  }
  #fullabs {
    background-color: rgba(0, 255, 0, 0.5);
    position: absolute;
    top: 0;
    left: 0;
    bottom: 0;
    right: 0;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

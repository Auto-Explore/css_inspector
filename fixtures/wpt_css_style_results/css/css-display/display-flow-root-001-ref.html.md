# css/css-display/display-flow-root-001-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-display/display-flow-root-001-ref.html"
}
```

## style[0]

```css

html,body {
  color:black; background-color:white; font:16px/1 monospace; padding:0; margin:0;
}

.float {
  float: left;
  width: 20px;
  height: 40px;
  background: pink;
}

.clearfix:after{content:".";display:block;height:0;clear:both;visibility:hidden;}

  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

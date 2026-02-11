# css/CSS2/margin-padding-clear/margin-collapse-clear-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-clear-001.xht"
}
```

## style[0]

```css

    div.target1 { position: absolute; left: -15px; height: 50px; width: 0; border-left: 10px solid yellow; top: 0px; }
    div.target2 { position: absolute; left: -15px; height: 20px; width: 0; border-left: 5px solid aqua; top: 50px; }
    div.target3 { position: absolute; left: -15px; height: 50px; width: 0; border-left: 10px solid orange; top: 100px; }
    div.container { width: 150px; position: relative; margin-left: 20px; border: solid thin; }
    div.box1 { height: 50px; background-color: yellow; }
    div.box2 { background-color: aqua; float: left; width: 20px; height: 20px; }
    div.box3 { margin-top: 50px; height: 50px; background-color: orange; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

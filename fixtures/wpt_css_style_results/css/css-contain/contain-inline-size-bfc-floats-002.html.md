# css/css-contain/contain-inline-size-bfc-floats-002.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/contain-inline-size-bfc-floats-002.html"
}
```

## style[0]

```css

  .float { float: left; background-color: blue; }
  .right { float: right; }

  #outer { width: 400px; }
  #float1 { width: 200px; height: 100px; }
  #float2 { width: 250px; height: 100px; }
  #float3 { width: 300px; height: 100px; }

  #contain {
    contain: inline-size;
    display: flow-root;
    width: fit-content;
  }
  #orange {
    display: inline-block;
    width: 300px;
    height: 20px;
    background: orange;
    vertical-align: top;
  }
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

# css/css-flexbox/flexible-box-float.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/flexible-box-float.html"
}
```

## style[0]

```css

        #myDiv {
            display: flex;
            flex-flow: row wrap;
            align-content: space-between;
            position: relative;
        }
        #myDiv p{
            width: 300px;
            height: 30px;
            margin: 0;
        }
        #first-p{
            background-color: green;
            float: right;
        }
        #second-p{
            background-color: blue;
        }
        #myDiv #fail-flag{
            width: 600px;
            background-color: red;
            position: absolute;
            left: 0;
            top: 0;
            z-index: -1;
        }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “flex-flow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

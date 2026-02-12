# css/css-transforms/transform-origin-01.html

```json
{
  "format_version": 3,
  "file": "css/css-transforms/transform-origin-01.html"
}
```

## style[0]

```css

        #container{
            position: relative;
        }

        .square{
            position: absolute;
        }

        #blue{
            top: 50px;
            left: 50px;
            width: 100px;
            height: 100px;
            background: blue;
        }

        #red{
            top: 0px;
            left: 100px;
            width: 100px;
            height: 100px;
            background: red;
            transform-origin: left center;
            transform: rotate(90deg);
        }

        #green{
            top: 0px;
            left: 100px;
            width: 100px;
            height: 100px;
            background: green;
            transform-origin: left;
            transform: rotate(90deg);
        }

        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

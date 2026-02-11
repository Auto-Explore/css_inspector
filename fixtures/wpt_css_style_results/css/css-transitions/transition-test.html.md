# css/css-transitions/transition-test.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/transition-test.html"
}
```

## style[0]

```css

        .container {
            background-color: red;
            height: 200px;
            width: 200px;
        }
        .box {
            width: 100px;
            height: 100px;
            background-color: green;

            transition-property: width, foo; /* invalid foo */
            transition-duration: 0, 5s;
        }
        .box.transition {
            width: 200px;
            height: 200px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

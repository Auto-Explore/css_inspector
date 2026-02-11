# css/css-shadow/css-scoping-shadow-host-with-before-after.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-host-with-before-after.html"
}
```

## style[0]

```css

        my-host {
            display: block;
            width: 100px;
            height: 100px;
            background: red;
        }
        my-host::before {
            display: block;
            content: "";
            width: 100px;
            height: 25px;
            background: green;
        }
        my-host::after {
            display: block;
            content: "";
            width: 100px;
            height: 25px;
            background: green;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

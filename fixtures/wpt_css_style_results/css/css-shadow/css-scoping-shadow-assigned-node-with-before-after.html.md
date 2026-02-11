# css/css-shadow/css-scoping-shadow-assigned-node-with-before-after.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-assigned-node-with-before-after.html"
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
        div {
            display: block;
            background: red;
            line-height: 0px;
            width: 100%;
            height: 50px;
        }
        [slot=foo]::before,
        [slot=foo]::after {
            display: block;
            content: "";
            width: 100%;
            height: 25px;
        }
        [slot=foo]::before,
        [slot=foo]::after {
            background: green;
        }
        [slot=bar]::before,
        [slot=bar]::after {
            background: yellow;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-shadow/css-scoping-shadow-slotted-rule.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-slotted-rule.html"
}
```

## style[0]

```css

        my-host {
            display: block;
            width: 100px;
            height: 100px;
            color: red;
            background: green;
        }
        my-host > div, nested-host {
            display: block;
            width: 100px;
            height: 25px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css
 ::slotted(.green), ::slotted(myelem) { color:green; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css
 .mydiv ::slotted(*) { color:green; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css
 ::slotted(*) { color:green; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

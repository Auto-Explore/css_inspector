# css/css-shadow/css-scoping-shadow-slotted-nested.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-slotted-nested.html"
}
```

## style[0]

```css

        outer-host {
            display: block;
            width: 100px;
            height: 100px;
            background: red;
        }
        outer-host > * {
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

                ::slotted([slot=outer]) { background: green; color: green; }
                ::slotted([slot=both]) { background: green; }
                span { display: block; width: 100px; height: 25px; }
            
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

            ::slotted([slot=inner]) { background: green; color: green; }
            ::slotted([slot=both]) { color: green; }
        
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

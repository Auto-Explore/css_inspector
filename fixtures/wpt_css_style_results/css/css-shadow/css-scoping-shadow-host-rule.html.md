# css/css-shadow/css-scoping-shadow-host-rule.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-host-rule.html"
}
```

## style[0]

```css

        my-host, my-host2, my-host3, my-host4 {
            display: block;
            width: 100px;
            height: 25px;
        }
        my-host2 {
            background: green;
        }
        my-host3 {
            background: red;
            color: green;
        }
        my-host4 {
            background: green;
            color: green;
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
 :host { color: green; background: green; } 
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
 :host { color: red; background: red; } div { color: green }
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
 :host { background: green !important; color: green !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css
 .container :host { background: red !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

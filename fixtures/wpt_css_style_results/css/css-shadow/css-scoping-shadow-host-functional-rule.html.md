# css/css-shadow/css-scoping-shadow-host-functional-rule.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-host-functional-rule.html"
}
```

## style[0]

```css

        host-1, host-2, host-3, host-4, host-5 {
            display: block;
            width: 100px;
            height: 20px;
            background: red;
        }
        host-3, host-4, host-5  {
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

## style[1]

```css
 :host(host-1) { background: green !important; } 
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
 :host(host-2.foo#bar[name=baz]) { background: green !important; } 
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
 :host(div host-3) { background: red !important; } 
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
 :host(.child) { background: red !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css
 :host(host-1) { background: red !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

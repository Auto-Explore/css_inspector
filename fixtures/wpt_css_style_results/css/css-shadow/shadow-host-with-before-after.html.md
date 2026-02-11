# css/css-shadow/shadow-host-with-before-after.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/shadow-host-with-before-after.html"
}
```

## style[0]

```css

.test {
    width: 100px;
    height: 25px;
    background: red;
    color: red;
}
#host1, #host2  {
    color: green;
}
#host3 div, #host4 div {
    width: 50%;
    height: 100%;
    background: green;
    display: inline-block;
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

    :host::before, :host::after {
        width: 50%;
        height: 100%;
        background: green;
        display: inline-block;
        content: "test";
    }
    
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

    :host(.green)::before, :host(.green)::after {
        width: 50%;
        height: 100%;
        background: green;
        display: inline-block;
        content: "test";
    }
    
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

    :host {
        color: green !important;
    }
    :host::before {
        width: 50%;
        height: 100%;
        background: green;
        display: inline-block;
        content: "test";
    }
    
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

    :host(.green) {
        color: green !important;
    }
    :host(.green)::after {
        width: 50%;
        height: 100%;
        background: green;
        display: inline-block;
        content: "test";
    }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

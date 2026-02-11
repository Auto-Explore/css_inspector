# css/css-shadow/css-scoping-shadow-host-namespace.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-host-namespace.html"
}
```

## style[0]

```css

        .host {
            display: block;
            width: 100px;
            height: 10px;
            background: red;
        }
        #host-3, #host-5, #host-10 {
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
@namespace url(http://www.w3.org/1999/xhtml); :host { background: green !important; } 
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
@namespace url(http://dummy); :host { background: green !important; } 
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
@namespace url(http://dummy); :host(*) { background: red !important; } 
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
@namespace url(http://dummy); :host(*|*) { background: green !important; } 
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
@namespace url(http://dummy); :host-context(*) { background: red !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[6]

```css
@namespace url(http://dummy); :host-context(*|*) { background: green !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[7]

```css
@namespace url(http://www.w3.org/1999/xhtml); :host(*) { background: green !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[8]

```css
@namespace url(http://www.w3.org/1999/xhtml); :host-context(*) { background: green !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[9]

```css
@namespace url(http://www.w3.org/1999/xhtml); :host(div) { background: green !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[10]

```css
@namespace url(http://dummy); :host-context(html) { background: red !important; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

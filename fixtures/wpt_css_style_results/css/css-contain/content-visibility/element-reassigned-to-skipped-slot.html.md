# css/css-contain/content-visibility/element-reassigned-to-skipped-slot.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/element-reassigned-to-skipped-slot.html"
}
```

## style[0]

```css

          slot[name=locked] {
            display: block;
            content-visibility: hidden;
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

#container {
  width: 150px;
  height: 150px;
  background: lightblue;
}

div {
  width: 50px;
  height: 50px;
}
.composited { will-change: transform; }
#one { background: pink; }
#two { background: red; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

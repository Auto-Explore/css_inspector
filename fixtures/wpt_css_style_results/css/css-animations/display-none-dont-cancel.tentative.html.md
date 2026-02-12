# css/css-animations/display-none-dont-cancel.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-animations/display-none-dont-cancel.tentative.html"
}
```

## style[0]

```css

@keyframes display1 {
  0% { display: none; }
  100% { display: inline; }
}
.animate1 {
  animation: display1 1s infinite;
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

@keyframes display2 {
  0% { display: var(--none-value); }
  100% { display: inline; }
}
.animate2 {
  animation: display2 1s infinite;
}
#target2 {
  --none-value: none;
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

@keyframes display3 {
  0% { display: none; }
  100% { display: none; }
}
.animate3 {
  animation: display3 1s infinite;
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

@keyframes display4 {
  0% { display: var(--none-value); }
  100% { display: var(--none-value); }
}
.animate4 {
  animation: display4 1s infinite;
}
#target4 {
  --none-value: none;
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

@keyframes display5 {
  0% { --display: none; }
  100% { --display: none; }
}
.animate5 {
  animation: display5 1s infinite;
}
#target5 {
  display: var(--display, block);
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

# css/css-shadow/shadow-cascade-order-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/shadow-cascade-order-001.html"
}
```

## style[0]

```css
my-item { color: red; }
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
::slotted(my-item) { color: blue; }
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
:host { color: yellow; }
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
' + slottedStyle + '
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
' + hostStyle + '
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
my-item { color: red; }
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
::slotted(my-item) { color: blue; }
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
:host { color: yellow; }
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
' + slottedStyle1 + '
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
' + slottedStyle2 + '
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
' + hostStyle + '
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

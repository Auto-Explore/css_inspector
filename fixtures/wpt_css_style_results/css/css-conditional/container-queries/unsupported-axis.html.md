# css/css-conditional/container-queries/unsupported-axis.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/unsupported-axis.html"
}
```

## style[0]

```css

  #container {
    width: 200px;
    height: 100px;
    container-type: inline-size;
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

  @container (width > 0px) {
    #target { --width:true; }
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

  @container (height > 0px) {
    #target { --height:true; }
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

  @container ((height > 0px) or (width > 0px)) {
    #target { --height-or-width:true; }
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

  @container ((width > 0px) or (height > 0px)) {
    #target { --width-or-height:true; }
  }
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

  @container ((orientation: landscape) or (width > 0px)) {
    #target { --orientation-or-width:true; }
  }
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

  @container ((width > 0px) or (orientation: landscape)) {
    #target { --width-or-orientation:true; }
  }
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

  @container ((height > 0px) or (orientation: landscape)) {
    #target { --height-or-orientation:true; }
  }
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

  @container ((height > 0px) or (orientation: landscape)) {
    #target { --height-or-orientation2:true; }
  }
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

  @container (inline-size > 0px) {
    #target { --inline-size:true; }
  }
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

  @container (block-size > 0px) {
    #target { --block-size:true; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[11]

```css

  @container (block-size > 0px) {
    #target { --block-size2:true; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[12]

```css

  @container not (width < 0px) {
    #target { --not-width:true; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[13]

```css

  @container not (height < 0px) {
    #target { --not-height:true; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[14]

```css

  @container not (inline-size < 0px) {
    #target { --not-inline-size:true; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[15]

```css

  @container not (block-size < 0px) {
    #target { --not-block-size:true; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[16]

```css

  @container not (orientation) {
    #target { --not-orientation:true; }
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

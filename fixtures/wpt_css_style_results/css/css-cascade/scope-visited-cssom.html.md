# css/css-cascade/scope-visited-cssom.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-visited-cssom.html"
}
```

## style[0]

```css

  :where(:visited, :link), :where(div) {
    background-color: white;
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

    @scope (main) {
      :link { background-color: green; }
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

    @scope (main) {
      :visited { background-color: green; }
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

    @scope (main) {
      :not(:link) { background-color: green; }
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

    @scope (main) {
      :not(:visited) { background-color: green; }
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

    @scope (main :link) {
      div { background-color: green; }
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

    @scope (main :visited) {
      div { background-color: green; }
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

    @scope (main :not(:visited)) {
      div { background-color: green; }
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

    @scope (main :not(:link)) {
      div { background-color: green; }
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

    @scope (main :link) {
      :scope { background-color: green; }
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

    @scope (main :visited) {
      :scope { background-color: green; }
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

    @scope (main :not(:visited)) {
      :scope { background-color: green; }
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

    @scope (main :not(:link)) {
      :scope { background-color: green; }
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

    @scope (main) to (:link) {
      * { background-color: green; }
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

    @scope (main) to (:visited) {
      * { background-color: green; }
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

    @scope (main) to (:not(:link)) {
      * { background-color: green; }
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

    @scope (main) to (:not(:visited)) {
      * { background-color: green; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

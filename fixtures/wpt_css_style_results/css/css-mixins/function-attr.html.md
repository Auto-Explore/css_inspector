# css/css-mixins/function-attr.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-attr.html"
}
```

## style[0]

```css

  #parent {
    list-style-image: url(parent);
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

    @function --f() {
      result: url(img.png);
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(img.png); }
  
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

    @function --f() {
      result: url("img.png");
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url("img.png"); }
  
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

    @function --f() returns <url> {
      result: url(img.png);
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(img.png); }
  
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

    @function --f() returns <url> {
      result: url("img.png");
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url("img.png"); }
  
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

    @function --f() {
      result: attr(data-42 type(<length>));
    }
    #actual { width: --f(); }
    #expected { width: 42px; }
  
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

    @function --f() returns <length> {
      result: attr(data-42 type(<length>));
    }
    #actual { width: --f(); }
    #expected { width: 42px; }
  
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

    @function --f() returns <length> {
      result: attr(data-42 type(*));
    }
    #actual { width: --f(); }
    #expected { width: 42px; }
  
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

    @function --f() {
      result: attr(data-42 type(*));
    }
    #actual { width: --f(); }
    #expected { width: 42px; }
  
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

    @function --f(--a : attr(data-42 type(*))) {
      result: var(--a);
    }
    #actual { width: --f(); }
    #expected { width: 42px; }
  
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

    @function --f() {
      --l: attr(data-42 type(*));
      result: var(--l);
    }
    #actual { width: --f(); }
    #expected { width: 42px; }
  
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

    @function --f() {
      result: attr(data-cat type(*));
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(parent); }
  
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

    @function --f() {
      result: attr(data-cat type(<url>));
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(parent); }
  
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

    @function --f() returns <url> {
      result: attr(data-cat type(*));
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(parent); }
  
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

    @function --f() returns <url> {
      --local: attr(data-cat type(*));
      result: var(--local);
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(parent); }
  
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

    @function --f(--arg) returns <url> {
      result: var(--arg);
    }
    #actual { list-style-image: --f(attr(data-cat type(*))); }
    #expected { list-style-image: url(parent); }
  
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

    @function --f(--arg: attr(data-cat type(*))) {
      result: var(--arg);
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(parent); }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[17]

```css

    @function --f() {
      --x: attr(data-cat type(*));
      result: --g();
    }
    @function --g() {
      result: var(--x);
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(parent); }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[18]

```css

    @function --f(--x: attr(data-cat type(*))) {
      --x: initial;
      result: --g();
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(parent); }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[19]

```css

    @function --f() {
      --x: attr(data-cat type(*));
      result: --g();
    }
    @function --g() {
      --x: inherit;
      result: var(--x);
    }
    #actual { list-style-image: --f(); }
    #expected { list-style-image: url(parent); }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

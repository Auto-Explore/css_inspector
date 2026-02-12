# css/css-mixins/function-shadow-animations.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/function-shadow-animations.html"
}
```

## style[0]

```css

  @property --length {
    syntax: "<length>";
    inherits: false;
    initial-value: 0px;
  }

  @function --from() { result: 1000px; }
  @function --to() { result: 2000px; }
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

      @keyframes --anim {
        from { width: --from(); }
        to { width: --to(); }
      }
      @function --from() { result: 0px; }
      @function --to() { result: 100px; }
      div {
        animation: --anim 1000s linear paused forwards;
      }
      #t00 { animation-delay: 0s; }
      #t04 { animation-delay: -400s; }
      #t05 { animation-delay: -500s; }
      #t06 { animation-delay: -600s; }
      #t10 { animation-delay: -1000s; }
    
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

      @keyframes --anim {
        from { --length: --from(); }
        to { --length: --to(); }
      }
      @function --from() { result: 0px; }
      @function --to() { result: 100px; }
      div {
        animation: --anim 1000s linear paused forwards;
      }
      #t00 { animation-delay: 0s; }
      #t04 { animation-delay: -400s; }
      #t05 { animation-delay: -500s; }
      #t06 { animation-delay: -600s; }
      #t10 { animation-delay: -1000s; }
    
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

      @keyframes --anim {
        from { --untyped: --from(); }
        to { --untyped: --to(); }
      }
      @function --from() { result: 0px; }
      @function --to() { result: 100px; }
      div {
        animation: --anim 1000s linear paused forwards;
      }
      #t00 { animation-delay: 0s; }
      #t04 { animation-delay: -400s; }
      #t05 { animation-delay: -500s; }
      #t06 { animation-delay: -600s; }
      #t10 { animation-delay: -1000s; }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

document.querySelectorAll('.tab').forEach(function(btn) {
  btn.addEventListener('click', function() {
    var tab = btn.getAttribute('data-tab');
    document.querySelectorAll('.tab').forEach(function(t) { t.classList.remove('active'); });
    document.querySelectorAll('.panel').forEach(function(p) { p.classList.remove('active'); });
    btn.classList.add('active');
    document.getElementById(tab).classList.add('active');
  });
});
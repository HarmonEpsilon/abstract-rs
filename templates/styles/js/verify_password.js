$('#password_box').on('change', function() {
  if ($(this).val() != $('#verify_password_box').val()) alert('Slow down. These do not match');
});
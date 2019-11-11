$(document).ready(function () {
        let reg_form = $(".registration_form");
        let reg_button = $(".register");
        reg_form.hide();
        reg_button.click(function () {
            reg_form.show();
        });
    }
);

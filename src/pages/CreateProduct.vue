<template>
    <div class="container mx-auto">
        <Card class="mt-8 w-1/3 mx-auto" :pt="cardClasses">
            <template #title>
                <h1 class="text-center">Create a product</h1>
            </template>

            <template #content>
                <form @submit.prevent="onSubmit" class="flex flex-col gap-2">
                    <div class="p-float-label mb-6 w-full">
                        <InputText id="name" v-model="name" :class="{ 'p-invalid': nameError }" class="w-full" aria-describedby="text-error"  type="text" />
                        <label for="name">Product Name</label>
                    </div>
                    <div class="p-float-label mb-6">
                        <InputNumber :class="{ 'p-invalid': priceError }" class="w-full" aria-describedby="text-error" id="price" placeholder="2.53" v-model="price" inputId="minmaxfraction" :minFractionDigits="2" :maxFractionDigits="2" />
                        <label for="price">Price</label>
                    </div>
                    <div class="p-float-label mb-6">
                        <Textarea id="description" v-model="description" :class="{ 'p-invalid': descriptionError }" rows="4" cols="30" class="w-full"
                            aria-describedby="text-error" />
                        <label for="description">Description</label>
                    </div>
                    <small id="text-error" class="p-error" v-if="!isFormValid">
                        <p v-for="error in errorBag">{{  error[0] }}</p>   
                    </small>
                    <Button type="submit" label="Submit" :disabled="!isFormValid" />
                </form>
            </template>
        </Card>
    </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router';
import { useCanister } from "@connect2ic/vue"
import { useField, useForm, useIsFormValid } from 'vee-validate';
import { Float32 } from '@dfinity/candid/lib/cjs/idl';


const cardClasses = { root: { class: `shadow-none border border-solid border-grey` } }
const route = useRoute()
const [backendService] = useCanister("backend")


const { handleSubmit, resetForm, errorBag } = useForm();
const isFormValid = useIsFormValid()
const { value: description, errorMessage : descriptionError } = useField('description', validateDescription);
const { value: price, errorMessage : priceError } = useField('price', validatePrice);
const { value: name, errorMessage : nameError } = useField('name', validateName);


function validateDescription(value) {
    if (!value) {
        return 'Description is required.';
    }

    if(value.length > 100) {
        return 'Description is too long'
    }

    return true;
}

function validatePrice(value) {
    const parsed = parseFloat(value)
    
    if(!parsed) {
        return 'Price should be a number'
    }

    if(parsed <= 0) {
        return 'Price must be more than 0'
    }

    return true
}

function validateName(value) {
    if(!value) {
        return "Name is required"
    }

    if(value.length > 100) {
        return "Name is too long"
    }

    return true
}



const onSubmit = handleSubmit((values) => {
 
    console.log(values)
    
    //  resetForm();
});


</script>

<style scoped></style>